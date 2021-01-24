use crate::types::uplay_overlapped::UplayOverlapped;
use std::{ffi::c_void, os::raw::c_char};

#[export_name = "UPLAY_OVERLAY_Show"]
pub fn uplay_overlay_show(_overlay_section: i32, _overlapped: *const UplayOverlapped) -> usize {
    debug!("UPLAY_OVERLAY_Show");
    return 0;
}

#[export_name = "UPLAY_OVERLAY_SetShopUrl"]
pub unsafe fn uplay_overlay_set_shop_url(
    _url_utf8: *const c_char,
    overlapped: *mut UplayOverlapped,
) -> usize {
    debug!("UPLAY_OVERLAY_SetShopUrl");

    if !overlapped.is_null() {
        (*overlapped).is_completed = 1;
        (*overlapped).reserved = 0;
    }

    return 1;
}

#[export_name = "UPLAY_OVERLAY_ShowShopUrl"]
pub fn uplay_overlay_show_shop_url(_url_utf8: *const c_void) -> usize {
    debug!("UPLAY_OVERLAY_ShowShopUrl");
    return 0;
}

#[export_name = "UPLAY_OVERLAY_ShowNotification"]
pub fn uplay_overlay_show_notification(_notification_id: *const c_void) -> usize {
    debug!("UPLAY_OVERLAY_ShowNotification");
    return 0;
}
