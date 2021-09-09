use serde::{Deserialize, Serialize};

/// Onebot Api 定义
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "action")]
pub enum Api {
    #[serde(rename = "send_private_msg")]
    SendPrivateMsg {
        params: SendPrivateMsg,
        echo: String,
    },
    #[serde(rename = "send_group_msg")]
    SendGroupMsg { params: SendGroupMsg, echo: String },
    #[serde(rename = "send_msg")]
    SendMsg { params: SendMsg, echo: String },
    #[serde(rename = "delete_msg")]
    DeleteMsg { params: DeleteMsg, echo: String },
    #[serde(rename = "get_msg")]
    GetMsg { params: GetMsg, echo: String },
    #[serde(rename = "get_forward_msg")]
    GetForwardMsg { params: GetForwardMsg, echo: String },
    #[serde(rename = "send_like")]
    SendLike { params: SendLike, echo: String },
    #[serde(rename = "set_group_kick")]
    SetGroupKick { params: SetGroupKick, echo: String },
    #[serde(rename = "set_group_ban")]
    SetGroupBan { params: SetGroupBan, echo: String },
    #[serde(rename = "set_group_anonymous_ban")]
    SetGroupAnonymousBan {
        params: SetGroupAnonymousBan,
        echo: String,
    },
    #[serde(rename = "set_group_whole_ban")]
    SetGroupWholeBan {
        params: SetGroupWholeBan,
        echo: String,
    },
    #[serde(rename = "set_group_admin")]
    SetGroupAdmin { params: SetGroupAdmin, echo: String },
    #[serde(rename = "set_group_anonymous")]
    SetGroupAnonymous {
        params: SetGroupAnonymous,
        echo: String,
    },
    #[serde(rename = "set_group_card")]
    SetGroupCard { params: SetGroupCard, echo: String },
    #[serde(rename = "set_group_name")]
    SetGroupName { params: SetGroupName, echo: String },
    #[serde(rename = "set_group_leave")]
    SetGroupLeave { params: SetGroupLeave, echo: String },
    #[serde(rename = "set_group_special_title")]
    SetGroupSpecialTitle {
        params: SetGroupSpecialTitle,
        echo: String,
    },
    #[serde(rename = "set_friend_add_request")]
    SetFriendAddRequest {
        params: SetFriendAddRequest,
        echo: String,
    },
    #[serde(rename = "set_group_add_request")]
    SetGroupAddRequest {
        params: SetGroupAddRequest,
        echo: String,
    },
    #[serde(rename = "get_login_info")]
    GetLoginInfo { params: Option<i8>, echo: String },
    #[serde(rename = "get_stranger_info")]
    GetStrangerInfo {
        params: GetStrangerInfo,
        echo: String,
    },
    #[serde(rename = "get_friend_list")]
    GetFriendList { params: Option<i8>, echo: String },
    #[serde(rename = "get_group_info")]
    GetGroupInfo { params: GetGroupInfo, echo: String },
    #[serde(rename = "get_group_list")]
    GetGroupList { params: Option<i8>, echo: String },
    #[serde(rename = "get_group_member_info")]
    GetGroupMemberInfo {
        params: GetGroupMemberInfo,
        echo: String,
    },
    #[serde(rename = "get_group_member_list")]
    GetGroupMemberList {
        params: GetGroupMemberList,
        echo: String,
    },
    #[serde(rename = "get_group_honor_info")]
    GetGroupHonorInfo {
        params: GetGroupHonorInfo,
        echo: String,
    },
    #[serde(rename = "get_cookies")]
    GetCookies { params: GetCookies, echo: String },
    #[serde(rename = "get_csrf_token")]
    GetCsrfToken { params: Option<i8>, echo: String },
    #[serde(rename = "get_credentials")]
    GetCredentials { params: GetCookies, echo: String },
    #[serde(rename = "get_record")]
    GetRecord { params: GetRecord, echo: String },
    #[serde(rename = "get_image")]
    GetImage { params: GetImage, echo: String },
    #[serde(rename = "can_send_image")]
    CanSendImage { params: Option<i8>, echo: String },
    #[serde(rename = "can_send_record")]
    CanSendRecord { params: Option<i8>, echo: String },
    #[serde(rename = "get_status")]
    GetStatus { params: Option<i8>, echo: String },
    #[serde(rename = "get_version_info")]
    GetVersionInfo { params: Option<i8>, echo: String },
    #[serde(rename = "set_restart")]
    SetRestart { params: SetRestart, echo: String },
    #[serde(rename = "clean_cache")]
    CleanCache { params: Option<i8>, echo: String },
}

macro_rules! echos {
    ($($x: tt),*) => {
        pub fn get_echo(&self) -> String {
            match self {
                $(Api::$x {
                    params: _,
                    echo: echo,
                } => echo.clone(),)*
            }
        }
    };
}

macro_rules! no_params_builder {
    ($(($fn_name: ident, $api_type: tt)),*) => {
        $(pub fn $fn_name() -> Api {
            Api::$api_type {
                params: None,
                echo: format!("{}-{}", stringify!($api_type), crate::utils::timestamp()),
            }
        })*
    };
}

macro_rules! params_builder {
    ($(($fn_name: ident, $api_type: tt)),*) => {
        $(pub fn $fn_name(params: $api_type) -> Api {
            Api::$api_type {
                params: params,
                echo: format!("{}-{}", stringify!($api_type), crate::utils::timestamp()),
            }
        })*
    };
}

impl Api {
    // Api::SendPrivateMsg {
    //     params: _,
    //     echo: echo,
    // } => echo.clone(),
    echos!(
        SendPrivateMsg,
        SendGroupMsg,
        SendMsg,
        DeleteMsg,
        GetMsg,
        GetForwardMsg,
        SendLike,
        SetGroupKick,
        SetGroupBan,
        SetGroupAnonymousBan,
        SetGroupWholeBan,
        SetGroupAdmin,
        SetGroupAnonymous,
        SetGroupCard,
        SetGroupName,
        SetGroupLeave,
        SetGroupSpecialTitle,
        SetFriendAddRequest,
        SetGroupAddRequest,
        GetLoginInfo,
        GetStrangerInfo,
        GetFriendList,
        GetGroupInfo,
        GetGroupList,
        GetGroupMemberInfo,
        GetGroupMemberList,
        GetGroupHonorInfo,
        GetCookies,
        GetCsrfToken,
        GetCredentials,
        GetRecord,
        GetImage,
        CanSendImage,
        CanSendRecord,
        GetStatus,
        GetVersionInfo,
        SetRestart,
        CleanCache
    );

    // pub fn get_group_list() -> Api {
    //     Api::GetGroupList {
    //         params: None,
    //         echo: format!("{},{}", "GetGroupList", crate::utils::timestamp()),
    //     }
    // }
    no_params_builder!(
        (get_login_info, GetLoginInfo),
        (get_friend_list, GetFriendList),
        (get_group_list, GetGroupList),
        (get_csrf_token, GetCsrfToken),
        (can_send_image, CanSendImage),
        (can_send_record, CanSendRecord),
        (get_status, GetStatus),
        (get_version_info, GetVersionInfo),
        (clean_cache, CleanCache)
    );

    // pub fn send_private_msg(params: SendPrivateMsg) -> Api {
    //     Api::SendPrivateMsg {
    //         params: params,
    //         echo: format!("{}-{}", "SendGroupMsg", crate::utils::timestamp()),
    //     }
    // }
    params_builder!(
        (send_private_msg, SendPrivateMsg),
        (send_group_msg, SendGroupMsg),
        (send_msg, SendMsg),
        (delete_msg, DeleteMsg),
        (get_msg, GetMsg),
        (get_forward_msg, GetForwardMsg),
        (send_like, SendLike),
        (set_group_kick, SetGroupKick),
        (set_group_ban, SetGroupBan),
        (set_group_anonymous_ban, SetGroupAnonymousBan),
        (set_group_whole_ban, SetGroupWholeBan),
        (set_group_admin, SetGroupAdmin),
        (set_group_anonymous, SetGroupAnonymous),
        (set_group_card, SetGroupCard),
        (set_group_name, SetGroupName),
        (set_group_leave, SetGroupLeave),
        (set_group_special_title, SetGroupSpecialTitle),
        (set_friend_add_request, SetFriendAddRequest),
        (set_group_add_request, SetGroupAddRequest),
        (get_stranger_info, GetStrangerInfo),
        (get_group_info, GetGroupInfo),
        (get_group_member_info, GetGroupMemberInfo),
        (get_group_member_list, GetGroupMemberList),
        (get_group_honor_info, GetGroupHonorInfo),
        (get_cookies, GetCookies),
        (get_credentials, GetCookies),
        (get_record, GetRecord),
        (get_image, GetImage),
        (set_restart, SetRestart)
    );
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SendPrivateMsg {
    pub user_id: String,
    pub message: Vec<crate::message::Message>,
    pub auto_escape: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SendGroupMsg {
    pub group_id: String,
    pub message: Vec<crate::message::Message>,
    pub auto_escape: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SendMsg {
    pub message_type: Option<String>,
    pub user_id: Option<String>,
    pub group_id: Option<String>,
    pub message: Vec<crate::message::Message>,
    pub auto_escape: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeleteMsg {
    pub message_id: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetMsg {
    pub message_id: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetForwardMsg {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SendLike {
    pub user_id: String,
    pub times: u8,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetGroupKick {
    pub group_id: String,
    pub user_id: String,
    pub reject_add_request: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetGroupBan {
    pub group_id: String,
    pub user_id: String,
    pub duration: i64, // 禁言时长，单位秒，0表示取消禁言
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetGroupAnonymousBan {
    pub group_id: String,
    pub anonymous: crate::event::Anoymous,
    pub flag: String,
    pub duration: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetGroupWholeBan {
    pub group_id: String,
    pub enable: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetGroupAdmin {
    pub group_id: String,
    pub user_id: String,
    pub enable: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetGroupAnonymous {
    pub group_id: String,
    pub enable: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetGroupCard {
    pub group_id: String,
    pub user_id: String,
    pub card: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetGroupName {
    pub group_id: String,
    pub group_name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetGroupLeave {
    pub group_id: String,
    pub is_dismiss: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetGroupSpecialTitle {
    pub group_id: String,
    pub user_id: String,
    pub special_title: String,
    pub duration: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetFriendAddRequest {
    pub flag: String,
    pub approve: bool,
    pub remark: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetGroupAddRequest {
    pub flag: String,
    pub sub_type: String,
    pub approve: bool,
    pub reason: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetStrangerInfo {
    pub user_id: String,
    pub no_cache: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetGroupInfo {
    pub group_id: String,
    pub no_cache: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetGroupMemberInfo {
    pub group_id: String,
    pub user_id: String,
    pub no_cache: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetGroupMemberList {
    pub group_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetGroupHonorInfo {
    pub group_id: String,
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetCookies {
    pub domain: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetRecord {
    pub file: String,
    pub out_format: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetImage {
    pub file: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetRestart {
    pub delay: i64,
}
