use std::{ffi::c_void, os::raw::c_char};

use crate::types::{uplay_list::UplayList, uplay_overlapped::UplayOverlapped};

#[export_name = "UPLAY_ACH_GetAchievementImage"]
pub fn uplay_ach_get_achievement_image(
    _achievement_id: u32,
    _out_image: *mut c_void,
    _overlapped: *mut UplayOverlapped,
) -> usize {
    debug!("UPLAY_ACH_GetAchievementImage");
    return 0;
}

#[export_name = "UPLAY_ACH_EarnAchievement"]
pub fn uplay_ach_earn_achievement(
    _achievement_id: u32,
    _overlapped: *mut UplayOverlapped,
) -> usize {
    debug!("UPLAY_ACH_EarnAchievement");
    return 1;
}

#[export_name = "UPLAY_ACH_Write"]
pub fn uplay_ach_write(_achievement: *const c_char) -> usize {
    debug!("UPLAY_ACH_Write");
    return 0;
}

#[export_name = "UPLAY_ACH_GetAchievements"]
pub fn uplay_ach_get_achievements(
    _filter: u32,
    _account_id_utf8_or_null_if_current_user: *const c_char,
    _out_achievement_list: *mut *mut UplayList,
    _overlapped: *mut UplayOverlapped,
) -> usize {
    debug!("UPLAY_ACH_GetAchievements");
    return 0;
}

#[export_name = "UPLAY_ACH_ReleaseAchievementList"]
pub fn uplay_ach_release_achievement_list(_list: *mut *mut UplayList) -> usize {
    debug!("UPLAY_ACH_ReleaseAchievementList");
    return 1;
}
