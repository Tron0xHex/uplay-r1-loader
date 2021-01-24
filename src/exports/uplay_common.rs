use crate::types::{uplay_list::UplayList, uplay_overlapped::UplayOverlapped};
use std::{ffi::c_void, os::raw::c_char};

#[export_name = "UPLAY_Start"]
pub fn uplay_start(_uplay_id: u32, _flags: u32) -> usize {
    debug!("UPLAY_Start");
    return 0;
}

#[export_name = "UPLAY_Startup"]
pub fn uplay_startup(
    _uplay_id: u32,
    _game_version: u32,
    _language_country_code_utf8: *const c_char,
) -> usize {
    debug!("UPLAY_Startup");
    return 0;
}

#[export_name = "UPLAY_Quit"]
pub fn uplay_quit() -> usize {
    debug!("UPLAY_Quit");
    return 0;
}

#[export_name = "UPLAY_HasOverlappedOperationCompleted"]
pub fn uplay_has_overlapped_operation_completed(overlapped: *const UplayOverlapped) -> usize {
    debug!("UPLAY_HasOverlappedOperationCompleted");

    unsafe {
        if !overlapped.is_null() {
            return ((*overlapped).is_completed != 0) as usize;
        }
    }

    return 0;
}

#[export_name = "UPLAY_Init"]
pub fn uplay_init() -> usize {
    debug!("UPLAY_Init");
    return 1;
}

#[export_name = "UPLAY_GetOverlappedOperationResult"]
pub fn uplay_get_overlapped_operation_result(
    overlapped: *const UplayOverlapped,
    out_result: *mut c_void,
) -> usize {
    debug!("UPLAY_GetOverlappedOperationResult");

    unsafe {
        if !overlapped.is_null() && (*overlapped).is_completed == 1 {
            *(out_result as *mut i32) = (*overlapped).reserved;
            return 1;
        }
    }

    return 0;
}

#[export_name = "UPLAY_Update"]
pub fn uplay_update() -> usize {
    debug!("UPLAY_Update");
    return 1;
}

#[export_name = "UPLAY_Release"]
pub fn uplay_release(list: *mut UplayList) -> usize {
    debug!("UPLAY_Release");

    if !list.is_null() {
        unsafe {
            Box::from_raw(list);
        }
    }

    return 1;
}

#[export_name = "UPLAY_GetNextEvent"]
pub fn uplay_get_next_event(_: *mut isize) -> usize {
    debug!("UPLAY_GetNextEvent");
    return 0;
}

#[export_name = "UPLAY_GetLastError"]
pub fn uplay_get_last_error(_out_error_string: *const c_char) -> usize {
    debug!("UPLAY_GetLastError");
    return 0;
}

#[export_name = "UPLAY_GetInstallationError"]
pub fn uplay_get_installation_error(_: *const c_char) -> usize {
    debug!("UPLAY_GetInstallationError");
    return 0;
}

#[export_name = "UPLAY_SetGameSession"]
pub fn uplay_set_game_session(
    _game_session_identifier: isize,
    _session_data: isize,
    _flags: u32,
) -> usize {
    debug!("UPLAY_SetGameSession");
    return 0;
}

#[export_name = "UPLAY_ClearGameSession"]
pub fn uplay_clear_game_session() {
    debug!("UPLAY_ClearGameSession");
}
