use crate::{
    global::SAVES_OPEN_OPTIONS,
    helpers::manifest::read_manifest,
    helpers::manifest::write_manifest,
    helpers::manifest::Error as ManifestError,
    helpers::save::get_saves,
    helpers::save::read_save,
    helpers::save::remove_save,
    helpers::save::write_save,
    models::manifest::Save,
    types::uplay_save::UplaySave,
    types::{
        uplay_list::{List, UplayList},
        uplay_overlapped::UplayOverlapped,
    },
};
use err_derive::Error;
use std::{
    ffi::c_void, ffi::CStr, ffi::CString, fs::OpenOptions, io::Error as IoError, os::raw::c_char,
    ptr, slice, str::Utf8Error,
};

#[export_name = "UPLAY_SAVE_GetSavegames"]
pub unsafe fn uplay_save_get_savegames(
    save_games_list: *mut *mut UplayList,
    overlapped: *mut UplayOverlapped,
) -> usize {
    debug!(
        "UPLAY_SAVE_GetSavegames -> SaveGameList: {:?} Overlapped: {:?}",
        save_games_list, overlapped
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

            *save_games_list = list;

            #[cfg(feature = "next-gen-api")]
            {
                (*overlapped).set_result(None);
            }

            #[cfg(not(feature = "next-gen-api"))]
            {
                (*overlapped).set_zeros();
                (*overlapped).set_result(Some(save_games_list as *const c_void));
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
    save_handle: *mut u32,
    overlapped: *mut UplayOverlapped,
) -> usize {
    debug!(
        "UPLAY_SAVE_Open -> SlotId: {} Mode: {} SaveHandle: {:?}, Overlapped: {:?}",
        slot_id, mode, save_handle, overlapped
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

    *save_handle = slot_id;

    #[cfg(feature = "next-gen-api")]
    {
        (*overlapped).set_result(None);
    }

    #[cfg(not(feature = "next-gen-api"))]
    {
        (*overlapped).set_zeros();
        (*overlapped).set_result(Some(save_handle as *const c_void));
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
    buffer: *mut *mut c_char,
    num_of_bytes_read: *mut usize,
    overlapped: *mut UplayOverlapped,
) -> usize {
    debug!("UPLAY_SAVE_Read");

    match read_save(slot_id, num_of_bytes_to_read, offset) {
        Ok((data, size)) => {
            ptr::copy(data.as_ptr() as *const c_char, *buffer, data.len());

            (*num_of_bytes_read) = size;
            (*overlapped).set_zeros();

            #[cfg(feature = "next-gen-api")]
            {
                (*overlapped).set_result(None);
            }

            #[cfg(not(feature = "next-gen-api"))]
            {
                (*overlapped).set_result(Some(num_of_bytes_read as *const c_void));
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
    buffer: *const *const c_char,
    overlapped: *mut UplayOverlapped,
) -> usize {
    debug!(
        "UPLAY_SAVE_Write -> SlotId: {} NumOfBytesToWrite: {} Buffer: {:?} Overlapped: {:?}",
        slot_id, num_of_bytes_to_write, buffer, overlapped
    );

    match SAVES_OPEN_OPTIONS.read().get(&slot_id) {
        Some(open_options) => {
            let buffer_u8 =
                slice::from_raw_parts(*buffer as *const u8, num_of_bytes_to_write as usize);

            match write_save(slot_id, open_options, num_of_bytes_to_write, buffer_u8) {
                Ok(_) => {
                    #[cfg(feature = "next-gen-api")]
                    {
                        (*overlapped).set_result(None);
                    }

                    #[cfg(not(feature = "next-gen-api"))]
                    {
                        (*overlapped).set_zeros();
                        (*overlapped).set_result(Some(buffer as *const c_void));
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
pub unsafe fn uplay_save_set_name(slot_id: u32, name_utf8: *const c_char) -> usize {
    debug!(
        "UPLAY_SAVE_SetName -> SlotId: {} NameUtf8: {:?}",
        slot_id, name_utf8
    );

    #[derive(Debug, Error)]
    enum Error {
        #[error(display = "Manifest error: {0:?}", _0)]
        Manifest(#[error(from)] ManifestError),
        #[error(display = "Utf8 error: {0:?}", _0)]
        Utf8(#[error(from)] Utf8Error),
        #[error(display = "Io error: {0:?}", _0)]
        Io(#[error(from)] IoError),
    }

    if let Err(err) = || -> Result<(), Error> {
        let mut manifest = read_manifest()?;

        let save_id = slot_id as i64;
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
            #[cfg(feature = "next-gen-api")]
            {
                (*overlapped).set_result(None);
            }

            #[cfg(not(feature = "next-gen-api"))]
            {
                (*overlapped).set_zeros();
                (*overlapped).set_result(Some(slot_id as *const c_void));
            }

            return 1;
        }
        Err(err) => {
            error!("{}", err);
        }
    }

    return 0;
}
