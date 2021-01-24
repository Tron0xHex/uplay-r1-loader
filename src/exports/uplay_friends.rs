use std::{ffi::c_void, os::raw::c_char};

use crate::types::{uplay_list::UplayList, uplay_overlapped::UplayOverlapped};

#[export_name = "UPLAY_FRIENDS_Init"]
pub fn uplay_friends_init(_flags: u32) -> usize {
    debug!("UPLAY_FRIENDS_Init");
    return 0;
}

#[export_name = "UPLAY_FRIENDS_GetFriendList"]
pub fn uplay_friends_get_friend_list(
    _friend_list_filter: u32,
    _out_friend_list: *mut *mut UplayList,
) -> usize {
    debug!("UPLAY_FRIENDS_GetFriendList");
    return 0;
}

#[export_name = "UPLAY_FRIENDS_RequestFriendship"]
pub fn uplay_friends_request_friendship(
    _search_string_utf88: *const c_char,
    _overlapped: *mut UplayOverlapped,
) -> usize {
    debug!("UPLAY_FRIENDS_RequestFriendship");
    return 0;
}

#[export_name = "UPLAY_FRIENDS_IsFriend"]
pub fn uplay_friends_is_friend(_account_id_utf8: *const c_char) -> usize {
    debug!("UPLAY_FRIENDS_IsFriend");
    return 0;
}

#[export_name = "UPLAY_FRIENDS_AddToBlackList"]
pub fn uplay_friends_add_to_black_list(
    _account_id_utf8: *const c_char,
    _overlapped: *mut UplayOverlapped,
) -> usize {
    debug!("UPLAY_FRIENDS_AddToBlackList");
    return 0;
}

#[export_name = "UPLAY_FRIENDS_IsBlackListed"]
pub fn uplay_friends_is_black_listed(_account_id_utf8: *const c_char) -> usize {
    debug!("UPLAY_FRIENDS_IsBlackListed");
    return 0;
}

#[export_name = "UPLAY_FRIENDS_ShowFriendSelectionUI"]
pub fn uplay_friends_show_friend_selection_ui(
    _account_id_filter_list_utf8: *const c_char,
    _account_id_filter_list_length: u32,
    _overlapped: *mut UplayOverlapped,
    _out_result: *mut c_void,
) -> usize {
    debug!("UPLAY_FRIENDS_ShowFriendSelectionUI");
    return 0;
}

#[export_name = "UPLAY_FRIENDS_EnableFriendMenuItem"]
pub fn uplay_friends_enable_friend_menu_item(
    _account_id_filter_list_utf8: *const c_char,
    _account_id_filter_list_length: u32,
) -> usize {
    debug!("UPLAY_FRIENDS_EnableFriendMenuItem");
    return 0;
}

#[export_name = "UPLAY_FRIENDS_DisableFriendMenuItem"]
pub fn uplay_friends_disable_friend_menu_item(_id: usize) -> usize {
    debug!("UPLAY_FRIENDS_DisableFriendMenuItem");
    return 0;
}

#[export_name = "UPLAY_FRIENDS_InviteToGame"]
pub fn uplay_friends_invite_to_game(
    _account_id_utf8: *const c_char,
    _overlapped: *mut UplayOverlapped,
) -> usize {
    debug!("UPLAY_FRIENDS_InviteToGame");
    return 0;
}
