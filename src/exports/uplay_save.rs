use crate::{
    global::SAVES_OPEN_OPTIONS,
    helpers::manifest::read_manifest,
    helpers::manifest::write_manifest,
    helpers::manifest::Error as ManifestError,
    helpers::save::get_saves,
    helpers::save::read_save,
    helpers::save::remove_save,
    helpers::{manifest::get_manifest_path, save::write_save},
    models::manifest::{Manifest, Save},
    types::uplay_save::UplaySave,
    types::{
        uplay_list::{List, UplayList},
        uplay_overlapped::UplayOverlapped,
    },
};
use std::{
    ffi::CStr, ffi::CString, fs::OpenOptions, io::Error as IoError, os::raw::c_char, ptr, slice,
    str::Utf8Error,
};
use thiserror::Error;

#[export_name = "UPLAY_SAVE_GetSavegames"]
pub unsafe fn uplay_save_get_savegames(
    out_games_list: *mut *mut UplayList,
    overlapped: *mut UplayOverlapped,
) -> usize {
    debug!(
        "UPLAY_SAVE_GetSavegames -> SaveGameList: {:?} Overlapped: {:?}",
        out_games_list, overlapped
    );

    match get_saves() {
        Ok(saves) => {
            let saves_size = saves.len();
            let uplay_saves = Box::into_raw(Box::new(Vec::new()));

            for (id, name, _) in saves {
                let name_c_str = (*Box::into_raw(Box::new(CString::new(name).unwrap()))).as_ptr();
                let uplay_save = Box::into_raw(Box::new(UplaySave::new(id, name_c_str)));

                (*uplay_saves).push(uplay_save)
            }

            let list = Box::into_raw(Box::new(UplayList {
                count: saves_size as u32,
                list: List {
                    saves: (*uplay_saves).as_ptr() as *const *const UplaySave,
                },
            }));

            *out_games_list = list;

            if !overlapped.is_null() {
                (*overlapped).set_result();
            }

            return 1;
        }
        Err(err) => {
            error!("{}", err);
        }
    }

    return 0;
}

#[export_name = "UPLAY_SAVE_Open"]
pub unsafe fn uplay_save_open(
    slot_id: u32,
    mode: u32,
    out_save_handle: *mut u32,
    overlapped: *mut UplayOverlapped,
) -> usize {
    debug!(
        "UPLAY_SAVE_Open -> SlotId: {} Mode: {} SaveHandle: {:?}, Overlapped: {:?}",
        slot_id, mode, out_save_handle, overlapped
    );

    let open_options = if mode == 1 {
        OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .clone()
    } else {
        OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .clone()
    };

    SAVES_OPEN_OPTIONS.write().insert(slot_id, open_options);

    *out_save_handle = slot_id;

    if !overlapped.is_null() {
        (*overlapped).set_result();
    }

    return 1;
}

#[export_name = "UPLAY_SAVE_Close"]
pub unsafe fn uplay_save_close(slot_id: u32) -> usize {
    debug!("UPLAY_SAVE_Close");

    SAVES_OPEN_OPTIONS.write().remove(&slot_id);
    return 1;
}

#[export_name = "UPLAY_SAVE_Read"]
pub unsafe fn uplay_save_read(
    slot_id: u32,
    num_of_bytes_to_read: u32,
    offset: u32,
    data: *mut *mut c_char,
    num_of_bytes_read: *mut usize,
    overlapped: *mut UplayOverlapped,
) -> usize {
    debug!("UPLAY_SAVE_Read");

    match read_save(slot_id, num_of_bytes_to_read, offset) {
        Ok((buffer, size)) => {
            ptr::copy(buffer.as_ptr() as *const c_char, *data, buffer.len());

            *num_of_bytes_read = size;

            if !overlapped.is_null() {
                (*overlapped).set_result();
            }

            return 1;
        }
        Err(err) => {
            error!("{}", err);
        }
    }

    return 0;
}

#[export_name = "UPLAY_SAVE_Write"]
pub unsafe fn uplay_save_write(
    slot_id: u32,
    num_of_bytes_to_write: u32,
    data: *const *const c_char,
    overlapped: *mut UplayOverlapped,
) -> usize {
    debug!(
        "UPLAY_SAVE_Write -> SlotId: {} NumOfBytesToWrite: {} Buffer: {:?} Overlapped: {:?}",
        slot_id, num_of_bytes_to_write, data, overlapped
    );

    match SAVES_OPEN_OPTIONS.read().get(&slot_id) {
        Some(open_options) => {
            let buffer = slice::from_raw_parts(*data as *const u8, num_of_bytes_to_write as usize);

            match write_save(slot_id, open_options, num_of_bytes_to_write, buffer) {
                Ok(_) => {
                    if !overlapped.is_null() {
                        (*overlapped).set_result();
                    }

                    return 1;
                }
                Err(err) => {
                    error!("{}", err);
                }
            }
        }
        None => {}
    }

    return 0;
}

#[export_name = "UPLAY_SAVE_SetName"]
pub unsafe fn uplay_save_set_name(save_handle: u32, name_utf8: *const c_char) -> usize {
    debug!(
        "UPLAY_SAVE_SetName -> SaveHandle: {} NameUtf8: {:?}",
        save_handle, name_utf8
    );

    #[derive(Debug, Error)]
    enum Error {
        #[error("Manifest error: {0:?}")]
        Manifest(#[from] ManifestError),
        #[error("Utf8 error: {0:?}")]
        Utf8(#[from] Utf8Error),
        #[error("Io error: {0:?}")]
        Io(#[from] IoError),
    }

    if let Err(err) = || -> Result<(), Error> {
        let mut manifest = if !get_manifest_path().exists() {
            Manifest { saves: Vec::new() }
        } else {
            read_manifest()?
        };

        let save_id = save_handle as i64;
        let save_name = CStr::from_ptr(name_utf8).to_str()?.to_string();

        match manifest.saves.iter_mut().find(|save| save.id == save_id) {
            Some(save) => {
                save.name = save_name;
            }
            None => manifest.saves.push(Save {
                id: save_id,
                name: save_name,
            }),
        }

        write_manifest(&manifest)?;
        Ok(())
    }() {
        error!("{}", err);
        return 0;
    }

    return 1;
}

#[export_name = "UPLAY_SAVE_Remove"]
pub unsafe fn uplay_save_remove(slot_id: u32, overlapped: *mut UplayOverlapped) -> usize {
    debug!("UPLAY_SAVE_Remove");

    match remove_save(slot_id) {
        Ok(_) => {
            if !overlapped.is_null() {
                (*overlapped).set_result();
            }

            return 1;
        }
        Err(err) => {
            error!("{}", err);
        }
    }

    return 0;
}
