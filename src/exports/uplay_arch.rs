use crate::types::{uplay_list::UplayList, uplay_overlapped::UplayOverlapped};

#[export_name = "UPLAY_ACH_GetAchievementImage"]
pub fn uplay_ach_get_achievement_image() -> usize {
    debug!("UPLAY_ACH_GetAchievementImage");
    return 0;
}

#[export_name = "UPLAY_ACH_EarnAchievement"]
pub fn uplay_ach_earn_achievement() -> usize {
    debug!("UPLAY_ACH_EarnAchievement");
    return 1;
}

#[export_name = "UPLAY_ACH_Write"]
pub fn uplay_ach_write() -> usize {
    debug!("UPLAY_ACH_Write");
    return 0;
}

#[export_name = "UPLAY_ACH_GetAchievements"]
pub fn uplay_ach_get_achievements(
    _: u32,
    _: u32,
    _: *mut *mut UplayList,
    _: *mut UplayOverlapped,
) -> usize {
    debug!("UPLAY_ACH_GetAchievements");
    return 0;
}

#[export_name = "UPLAY_ACH_ReleaseAchievementList"]
pub fn uplay_ach_release_achievement_list(_: *mut *mut UplayList) -> usize {
    debug!("UPLAY_ACH_ReleaseAchievementList");
    return 1;
}
