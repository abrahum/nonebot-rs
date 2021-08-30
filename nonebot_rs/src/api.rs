use serde::{Deserialize, Serialize};

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
        params: GetGroupMemberList,
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
    GetRestart { params: GetRestart, echo: String },
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
            } => echo.clone(),
        )*
    }
}
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
        GetRestart,
        CleanCache
    );
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SendPrivateMsg {
    pub user_id: i64,
    pub message: Vec<crate::message::Message>,
    pub auto_escape: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SendGroupMsg {
    pub group_id: i64,
    pub message: Vec<crate::message::Message>,
    pub auto_escape: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SendMsg {
    pub message_type: Option<String>,
    pub user_id: Option<i64>,
    pub group_id: Option<i64>,
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
    pub user_id: i64,
    pub times: u8,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetGroupKick {
    pub group_id: i64,
    pub user_id: i64,
    pub reject_add_request: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetGroupBan {
    pub group_id: i64,
    pub user_id: i64,
    pub duration: i64, // 禁言时长，单位秒，0表示取消禁言
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetGroupAnonymousBan {
    group_id: i64,
    anonymous: crate::event::Anoymous,
    flag: String,
    duration: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetGroupWholeBan {
    group_id: i64,
    enable: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetGroupAdmin {
    group_id: i64,
    user_id: i64,
    enable: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetGroupAnonymous {
    group_id: i64,
    enable: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetGroupCard {
    group_id: i64,
    user_id: i64,
    card: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetGroupName {
    group_id: i64,
    group_name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetGroupLeave {
    group_id: i64,
    is_dismiss: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetGroupSpecialTitle {
    group_id: i64,
    user_id: i64,
    special_title: String,
    duration: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetFriendAddRequest {
    flag: String,
    approve: bool,
    remark: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetGroupAddRequest {
    flag: String,
    sub_type: String,
    approve: bool,
    reason: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetStrangerInfo {
    user_id: i64,
    no_cache: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetGroupInfo {
    group_id: i64,
    no_cache: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetGroupMemberInfo {
    group_id: i64,
    user_id: i64,
    no_cache: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetGroupMemberList {
    group_id: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetGroupHonorInfo {
    group_id: i64,
    #[serde(rename = "type")]
    type_: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetCookies {
    domain: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetRecord {
    file: String,
    out_format: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetImage {
    file: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetRestart {
    delay: i64,
}
