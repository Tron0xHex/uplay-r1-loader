use std::{ffi::c_void, ffi::CString, os::raw::c_char};

use crate::{global::CONFIG, types::uplay_list::UplayList};

lazy_static! {
    static ref LANGUAGE: CString = CString::new(CONFIG.uplay.language.clone()).unwrap();
}

#[export_name = "UPLAY_INSTALLER_Init"]
pub fn uplay_installer_init(_flags: u32) -> usize {
    debug!("UPLAY_INSTALLER_Init");
    return 1;
}

#[export_name = "UPLAY_INSTALLER_UpdateInstallOrder"]
pub fn uplay_installer_update_install_order(_chunk_ids: *const c_void, _chunk_count: u32) -> usize {
    debug!("UPLAY_INSTALLER_UpdateInstallOrder");
    return 1;
}

#[export_name = "UPLAY_INSTALLER_AreChunksInstalled"]
pub fn uplay_installer_are_chunks_installed(chunk_ids: *const c_void, _chunk_count: u32) -> usize {
    debug!("UPLAY_INSTALLER_AreChunksInstalled");
    return !chunk_ids.is_null() as usize;
}

#[export_name = "UPLAY_INSTALLER_GetChunks"]
pub fn uplay_installer_get_chunks(_out_chunk_id_list: *mut *mut UplayList) -> usize {
    debug!("UPLAY_INSTALLER_GetChunks");
    return 1;
}

#[export_name = "UPLAY_INSTALLER_GetChunkIdsFromTag"]
pub fn uplay_installer_get_chunk_ids_from_tag(
    _tag_utf8: *const c_char,
    _out_chunk_id_list: *mut *mut UplayList,
) -> usize {
    debug!("UPLAY_INSTALLER_GetChunkIdsFromTag");
    return 1;
}

#[export_name = "UPLAY_INSTALLER_GetLanguageUtf8"]
pub fn uplay_installer_get_language_utf8() -> *const c_char {
    debug!("UPLAY_INSTALLER_GetLanguageUtf8");
    return LANGUAGE.as_ptr();
}
