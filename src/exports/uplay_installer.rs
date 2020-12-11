use std::{ffi::c_void, ffi::CString, os::raw::c_char};

use crate::global::CONFIG;

lazy_static! {
    static ref LANGUAGE: CString = CString::new(CONFIG.uplay.language.clone()).unwrap();
}

#[export_name = "UPLAY_INSTALLER_Init"]
pub fn uplay_installer_init() -> usize {
    debug!("UPLAY_INSTALLER_Init");
    return 1;
}

#[export_name = "UPLAY_INSTALLER_UpdateInstallOrder"]
pub fn uplay_installer_update_install_order() -> usize {
    debug!("UPLAY_INSTALLER_UpdateInstallOrder");
    return 1;
}

#[export_name = "UPLAY_INSTALLER_AreChunksInstalled"]
pub fn uplay_installer_are_chunks_installed(a1: i32, a2: i32) -> usize {
    debug!("UPLAY_INSTALLER_AreChunksInstalled -> A1: {} A2: {}", a1, a2);
    return (a2 != 0) as usize;
}

#[export_name = "UPLAY_INSTALLER_GetChunks"]
pub fn uplay_installer_get_chunks(_: *const c_void) -> usize {
    debug!("UPLAY_INSTALLER_GetChunks");
    return 1;
}

#[export_name = "UPLAY_INSTALLER_GetChunkIdsFromTag"]
pub fn uplay_installer_get_chunk_ids_from_tag() -> usize {
    debug!("UPLAY_INSTALLER_GetChunkIdsFromTag");
    return 1;
}

#[export_name = "UPLAY_INSTALLER_GetLanguageUtf8"]
pub fn uplay_installer_get_language_utf8() -> *const c_char {
    debug!("UPLAY_INSTALLER_GetLanguageUtf8");
    return LANGUAGE.as_ptr() as *const c_char;
}
