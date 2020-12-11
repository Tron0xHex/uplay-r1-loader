use crate::types::{uplay_list::UplayList, uplay_overlapped::UplayOverlapped};
use std::{ffi::c_void, os::raw::c_char};

#[export_name = "UPLAY_Startup"]
pub fn uplay_startup() -> usize {
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
    result: *mut c_void,
) -> usize {
    debug!("UPLAY_GetOverlappedOperationResult");

    unsafe {
        if !overlapped.is_null() && (*overlapped).is_completed == 1 {
            *(result as *mut i32) = (*overlapped).reserved;
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
pub fn uplay_get_next_event(_: *mut i32) -> usize {
    debug!("UPLAY_GetNextEvent");
    return 0;
}

#[export_name = "UPLAY_GetLastError"]
pub fn uplay_get_last_error(_: *const c_char) -> usize {
    debug!("UPLAY_GetLastError");
    return 0;
}

#[export_name = "UPLAY_GetInstallationError"]
pub fn uplay_get_installation_error(_: *const c_char) -> usize {
    debug!("UPLAY_GetInstallationError");
    return 0;
}

#[export_name = "UPLAY_SetGameSession"]
pub fn uplay_set_game_session(_: i32, _: i32, _: i32, _: i32) -> usize {
    debug!("UPLAY_SetGameSession");
    return 0;
}

#[export_name = "UPLAY_ClearGameSession"]
pub fn uplay_clear_game_session() -> usize {
    debug!("UPLAY_ClearGameSession");
    return 1;
}
