use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiResp {
    pub status: String,
    pub retcode: i32,
    pub data: RespData,
    pub echo: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RespData {
    message_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "action")]
pub enum Apis {
    #[serde(rename = "send_private_msg")]
    SendPrivateMsg {
        params: SendPrivateMsg,
        echo: String,
    },
    #[serde(rename = "send_group_msg")]
    SendGroupMsg {
        params: SendGroupMsg,
        echo: String,
    },
    None,
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
