use serde::{Deserialize, Serialize};

/// Onebot Api 响应
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiResp {
    pub status: String,
    pub retcode: i32,
    pub data: RespData,
    pub echo: String,
}

/// Onebot Api 响应 data 字段
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct RespData {
    pub message_id: Option<i32>,
    pub time: Option<i32>,
    pub message_type: Option<String>,
    pub real_id: Option<i32>,
    pub sender: Option<Sender>,
    pub message: Option<Vec<crate::message::Message>>,
}

/// Onebot Api 响应 sender 字段
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Sender {
    pub user_id: i64,          // 发送者 QQ 号
    pub nickname: String,      // 昵称
    pub card: Option<String>,  // 群名片|备注
    pub sex: String,           // 性别 male|female|unkown
    pub age: i32,              // 年龄
    pub area: Option<String>,  // 地区
    pub level: Option<String>, // 成员等级
    pub role: Option<String>,  // 角色 owner|admin|member
    pub title: Option<String>, // 专属头衔
}

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
