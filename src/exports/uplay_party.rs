use std::{ffi::c_void, os::raw::c_char};

use crate::types::{uplay_list::UplayList, uplay_overlapped::UplayOverlapped};

#[export_name = "UPLAY_PARTY_Init"]
pub fn uplay_party_init(_flags: u32) -> usize {
    debug!("UPLAY_PARTY_Init");
    return 1;
}

#[export_name = "UPLAY_PARTY_InviteToParty"]
pub fn uplay_party_invite_to_party(
    _account_id_utf8: *const c_char,
    _overlapped: *mut UplayOverlapped,
) -> usize {
    debug!("UPLAY_PARTY_InviteToParty");
    return 0;
}

#[export_name = "UPLAY_PARTY_RespondToGameInvite"]
pub fn uplay_party_respond_to_game_invite(_invitation_id: u32, _accept: bool) -> usize {
    debug!("UPLAY_PARTY_RespondToGameInvite");
    return 0;
}

#[export_name = "UPLAY_PARTY_ShowGameInviteOverlayUI"]
pub fn uplay_party_show_game_invite_overlay_ui(_invitation_id: u32) -> usize {
    debug!("UPLAY_PARTY_ShowGameInviteOverlayUI");
    return 0;
}

#[export_name = "UPLAY_PARTY_GetInGameMemberList"]
pub fn uplay_party_get_in_game_member_list(_out_member_list: *mut *mut UplayList) -> usize {
    debug!("UPLAY_PARTY_GetInGameMemberList");
    return 0;
}

#[export_name = "UPLAY_PARTY_GetFullMemberList"]
pub fn uplay_party_get_full_member_list(_out_member_list: *mut *mut UplayList) -> usize {
    debug!("UPLAY_PARTY_GetFullMemberList");
    return 0;
}

#[export_name = "UPLAY_PARTY_SetUserData"]
pub fn uplay_party_set_user_data(_data_blob: *const c_void) -> usize {
    debug!("UPLAY_PARTY_SetUserData");
    return 0;
}

#[export_name = "UPLAY_PARTY_IsInParty"]
pub fn uplay_party_is_in_party(_account_id_utf8: *const c_char) -> usize {
    debug!("UPLAY_PARTY_IsInParty");
    return 0;
}

#[export_name = "UPLAY_PARTY_IsPartyLeader"]
pub fn uplay_party_is_party_leader(_account_id_utf8: *const c_char) -> usize {
    debug!("UPLAY_PARTY_IsPartyLeader");
    return 0;
}

#[export_name = "UPLAY_PARTY_PromoteToLeader"]
pub fn uplay_party_promote_to_leader(
    _account_id_utf8: *const c_char,
    _overlapped: *mut UplayOverlapped,
) -> usize {
    debug!("UPLAY_PARTY_PromoteToLeader");
    return 0;
}

#[export_name = "UPLAY_PARTY_InvitePartyToGame"]
pub fn uplay_party_invite_party_to_game(_overlapped: *mut UplayOverlapped) -> usize {
    debug!("UPLAY_PARTY_InvitePartyToGame");
    return 0;
}

#[export_name = "UPLAY_PARTY_EnablePartyMemberMenuItem"]
pub fn uplay_party_enable_party_member_menu_item(_: usize, _: usize, _: *const usize) -> usize {
    debug!("UPLAY_PARTY_EnablePartyMemberMenuItem");
    return 0;
}

#[export_name = "UPLAY_PARTY_DisablePartyMemberMenuItem"]
pub fn uplay_party_disable_party_member_menu_item() -> usize {
    debug!("UPLAY_PARTY_DisablePartyMemberMenuItem");
    return 0;
}

#[export_name = "UPLAY_PARTY_SetGuest"]
pub fn uplay_party_set_guest(_: *const c_char, _overlapped: *mut UplayOverlapped) -> usize {
    debug!("UPLAY_PARTY_SetGuest");
    return 0;
}
