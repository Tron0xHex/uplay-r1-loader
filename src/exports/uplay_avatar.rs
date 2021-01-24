use std::{ffi::c_void, os::raw::c_char};

use crate::types::uplay_overlapped::UplayOverlapped;

#[export_name = "UPLAY_AVATAR_GetBitmap"]
pub fn uplay_avatar_get_bitmap(
    _avatar_id: *const u32,
    _avatar_size: u32,
    _out_rgba: *mut c_void,
    _overlapped: *mut UplayOverlapped,
) -> usize {
    debug!("UPLAY_AVATAR_GetBitmap");
    return 0;
}

#[export_name = "UPLAY_AVATAR_GetAvatarIdForCurrentUser"]
pub fn uplay_avatar_get_avatar_id_for_current_user(
    _out_avatar_id: *mut isize,
    _overlapped: *mut UplayOverlapped,
) -> usize {
    debug!("UPLAY_AVATAR_GetAvatarIdForCurrentUser");
    return 0;
}

#[export_name = "UPLAY_AVATAR_Get"]
pub fn uplay_avatar_get(
    _account_id_utf8: *const c_char,
    _avatar_size: u32,
    _out_rgba: *mut c_void,
    _overlapped: *mut UplayOverlapped,
) -> usize {
    debug!("UPLAY_AVATAR_Get");
    return 0;
}
