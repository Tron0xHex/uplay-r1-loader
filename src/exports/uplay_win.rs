use std::ffi::c_void;

use crate::types::{uplay_list::UplayList, uplay_overlapped::UplayOverlapped};

#[export_name = "UPLAY_WIN_GetRewards"]
pub fn uplay_win_get_rewards(
    _out_reward_list: *mut *mut UplayList,
    _overlapped: *mut UplayOverlapped,
) {
    debug!("UPLAY_WIN_GetRewards");
}

#[export_name = "UPLAY_WIN_RefreshActions"]
pub fn uplay_win_refresh_actions() -> usize {
    debug!("UPLAY_WIN_RefreshActions");
    return 1;
}

#[export_name = "UPLAY_WIN_SetActionsCompleted"]
pub fn uplay_win_set_actions_completed(
    _action_ids_utf8: *const c_void,
    _action_ids_count: isize,
    _overlapped: *mut UplayOverlapped,
) -> usize {
    debug!("UPLAY_WIN_SetActionsCompleted");
    return 1;
}
