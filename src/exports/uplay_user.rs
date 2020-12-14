use crate::{
    global::CONFIG,
    types::{
        uplay_key::UplayKey,
        uplay_list::{List, UplayList},
        uplay_overlapped::UplayOverlapped,
    },
};
use std::{ffi::c_void, ffi::CString, os::raw::c_char, ptr};

lazy_static! {
    static ref ACCOUNT_ID: CString = CString::new(CONFIG.uplay.profile.account_id.clone()).unwrap();
    static ref USERNAME: CString = CString::new(CONFIG.uplay.profile.username.clone()).unwrap();
    static ref EMAIL: CString = CString::new(CONFIG.uplay.profile.email.clone()).unwrap();
    static ref PASSWORD: CString = CString::new(CONFIG.uplay.profile.password.clone()).unwrap();
}

#[export_name = "UPLAY_USER_GetCdKeys"]
pub unsafe fn uplay_user_get_cd_keys(
    cd_keys_list: *mut *mut UplayList,
    overlapped: *mut UplayOverlapped,
) -> usize {
    debug!(
        "UPLAY_USER_GetCdKeys -> CdKeysList: {:?} Overlapped: {:?}",
        cd_keys_list, overlapped
    );

    let keys_size = CONFIG.uplay.cd_keys.len();
    let uplay_keys = Box::into_raw(Box::new(Vec::new()));

    for key in &CONFIG.uplay.cd_keys {
        let uplay_key = Box::into_raw(Box::new(UplayKey {
            cd_key: (*Box::into_raw(Box::new(CString::new(key.clone()).unwrap()))).as_ptr(),
        }));

        (*uplay_keys).push(uplay_key)
    }

    let list = Box::into_raw(Box::new(UplayList {
        count: keys_size as u32,
        list: List {
            keys: (*uplay_keys).as_ptr() as *const *const UplayKey,
        },
    }));

    *cd_keys_list = list;

    #[cfg(feature = "next-gen-api")]
    {
        (*overlapped).set_result();
    }

    #[cfg(not(feature = "next-gen-api"))]
    {
        (*overlapped).set_zeros();
        (*overlapped).set_result(cd_keys_list as *const c_void);
    }

    return 1;
}

#[export_name = "UPLAY_USER_ReleaseCdKeyList"]
pub fn uplay_user_release_cd_key_list(cd_key_list: *mut UplayList) -> usize {
    debug!("UPLAY_USER_ReleaseCdKeyList");

    if !cd_key_list.is_null() {
        unsafe {
            Box::from_raw(cd_key_list);
        }
    }

    return 1;
}

#[export_name = "UPLAY_USER_GetCredentials"]
pub unsafe fn uplay_user_get_credentials(
    credentials: *mut c_void,
    overlapped: *mut UplayOverlapped,
) -> usize {
    debug!("UPLAY_USER_GetCredentials");

    ptr::copy(
        USERNAME.as_ptr() as *const c_void,
        credentials,
        USERNAME.as_bytes().len() + 1,
    );

    #[cfg(feature = "next-gen-api")]
    {
        (*overlapped).set_result();
    }

    #[cfg(not(feature = "next-gen-api"))]
    {
        (*overlapped).set_zeros();
        (*overlapped).set_result(credentials as *const c_void);
    }

    return 0;
}

#[export_name = "UPLAY_USER_GetCdKeyUtf8"]
pub fn uplay_user_get_cd_key_utf8() -> usize {
    debug!("UPLAY_USER_GetCdKeyUtf8");
    return 0;
}

#[export_name = "UPLAY_USER_GetAccountIdUtf8"]
pub fn uplay_user_get_account_id_utf8() -> *const c_char {
    debug!("UPLAY_USER_GetAccountIdUtf8");
    return ACCOUNT_ID.as_ptr() as *const c_char;
}

#[export_name = "UPLAY_USER_GetUsernameUtf8"]
pub fn uplay_user_get_username_utf8() -> *const c_char {
    debug!("UPLAY_USER_GetUsernameUtf8");
    return USERNAME.as_ptr() as *const c_char;
}

#[export_name = "UPLAY_USER_GetNameUtf8"]
pub fn uplay_user_get_name_utf8() -> *const c_char {
    debug!("UPLAY_USER_GetNameUtf8");
    return USERNAME.as_ptr() as *const c_char;
}

#[export_name = "UPLAY_USER_GetEmailUtf8"]
pub fn uplay_user_get_email_utf8() -> *const c_char {
    debug!("UPLAY_USER_GetEmailUtf8");
    return EMAIL.as_ptr() as *const c_char;
}

#[export_name = "UPLAY_USER_GetPasswordUtf8"]
pub fn uplay_user_get_password_utf8() -> *const c_char {
    debug!("UPLAY_USER_GetPasswordUtf8");
    return PASSWORD.as_ptr() as *const c_char;
}

#[export_name = "UPLAY_USER_GetAccountId"]
pub fn uplay_user_get_account_id() -> *const c_char {
    debug!("UPLAY_USER_GetAccountId");
    return ACCOUNT_ID.as_ptr() as *const c_char;
}

#[export_name = "UPLAY_USER_GetUsername"]
pub fn uplay_user_get_username() -> *const c_char {
    debug!("UPLAY_USER_GetUsername");
    return USERNAME.as_ptr() as *const c_char;
}

#[export_name = "UPLAY_USER_GetEmail"]
pub fn uplay_user_get_email() -> *const c_char {
    debug!("UPLAY_USER_GetEmail");
    return EMAIL.as_ptr() as *const c_char;
}

#[export_name = "UPLAY_USER_GetPassword"]
pub fn uplay_user_get_password() -> *const c_char {
    debug!("UPLAY_USER_GetPassword");
    return PASSWORD.as_ptr() as *const c_char;
}

#[export_name = "UPLAY_USER_IsConnected"]
pub fn uplay_user_is_connected() -> usize {
    debug!("UPLAY_USER_IsConnected");
    return 0;
}

#[export_name = "UPLAY_USER_IsOwned"]
pub fn uplay_user_is_owned() -> usize {
    debug!("UPLAY_USER_IsOwned");
    return 1;
}

#[export_name = "UPLAY_USER_IsInOfflineMode"]
pub fn uplay_user_is_in_offline_mode() -> usize {
    debug!("UPLAY_USER_IsInOfflineMode");
    return CONFIG.uplay.offline_mode as usize;
}

#[export_name = "UPLAY_USER_GetTicketUtf8"]
pub fn uplay_user_get_ticket_utf8() -> usize {
    debug!("UPLAY_USER_GetTicketUtf8");
    return 0;
}

#[export_name = "UPLAY_USER_ConsumeItem"]
pub fn uplay_user_consume_item() -> usize {
    debug!("UPLAY_USER_ConsumeItem");
    return 0;
}

#[export_name = "UPLAY_USER_GetConsumeItem"]
pub fn uplay_user_get_consume_item() -> usize {
    debug!("UPLAY_USER_GetConsumeItem");
    return 0;
}

#[export_name = "UPLAY_USER_GetConsumableItems"]
pub fn uplay_user_get_consumable_items() -> usize {
    debug!("UPLAY_USER_GetConsumableItems");
    return 0;
}

#[export_name = "UPLAY_USER_ReleaseConsumeItemResult"]
pub fn uplay_user_release_consume_item_result() -> usize {
    debug!("UPLAY_USER_ReleaseConsumeItemResult");
    return 0;
}

#[export_name = "UPLAY_USER_SetGameSession"]
pub fn uplay_user_set_game_session() -> usize {
    debug!("UPLAY_USER_SetGameSession");
    return 0;
}

#[export_name = "UPLAY_USER_ClearGameSession"]
pub fn uplay_user_clear_game_session() -> usize {
    debug!("UPLAY_USER_ClearGameSession");
    return 1;
}

#[export_name = "UPLAY_USER_GetGPUScoreConfidenceLevel"]
pub fn uplay_user_get_gpu_score_confidence_level() -> usize {
    debug!("UPLAY_USER_GetGPUScoreConfidenceLevel");
    return 0;
}

#[export_name = "UPLAY_USER_GetGPUScore"]
pub fn uplay_user_get_gpu_score() -> usize {
    debug!("UPLAY_USER_GetGPUScore");
    return 0;
}

#[export_name = "UPLAY_USER_GetCPUScore"]
pub fn uplay_user_get_cpu_score() -> usize {
    debug!("UPLAY_USER_GetCPUScore");
    return 0;
}
