#[export_name = "UPLAY_WIN_GetRewards"]
pub fn uplay_win_get_rewards() -> usize {
    debug!("UPLAY_WIN_GetRewards");
    return 0;
}

#[export_name = "UPLAY_WIN_RefreshActions"]
pub fn uplay_win_refresh_actions() -> usize {
    debug!("UPLAY_WIN_RefreshActions");
    return 1;
}

#[export_name = "UPLAY_WIN_SetActionsCompleted"]
pub fn uplay_win_set_actions_completed() -> usize {
    debug!("UPLAY_WIN_SetActionsCompleted");
    return 1;
}
