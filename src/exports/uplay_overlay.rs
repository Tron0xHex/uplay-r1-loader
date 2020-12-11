use crate::types::uplay_overlapped::UplayOverlapped;
use std::ffi::c_void;

#[export_name = "UPLAY_OVERLAY_Show"]
pub fn uplay_overlay_show(_: i32, _: *const UplayOverlapped) -> usize {
    debug!("UPLAY_OVERLAY_Show");
    return 0;
}

#[export_name = "UPLAY_OVERLAY_SetShopUrl"]
pub unsafe fn uplay_overlay_set_shop_url(
    _: *const c_void,
    overlapped: *mut UplayOverlapped,
) -> usize {
    debug!("UPLAY_OVERLAY_SetShopUrl");

    if !overlapped.is_null() {
        (*overlapped).reserved = 0;
        (*overlapped).is_completed = 1;
    }

    return 0;
} 

#[export_name = "UPLAY_OVERLAY_ShowShopUrl"]
pub fn uplay_overlay_show_shop_url(_: *const c_void, _: *mut UplayOverlapped) -> usize {
    debug!("UPLAY_OVERLAY_ShowShopUrl");
    return 0;
}

#[export_name = "UPLAY_OVERLAY_ShowNotification"]
pub fn uplay_overlay_show_notification(_: *const c_void, _: *const UplayOverlapped) -> usize {
    debug!("UPLAY_OVERLAY_ShowNotification");
    return 0;
}
