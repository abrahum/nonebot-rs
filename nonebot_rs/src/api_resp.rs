use crate::utils::id_deserializer;
use serde::{Deserialize, Serialize};

/// Onebot Api 响应根结构体
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiResp {
    pub status: String,
    pub retcode: i32,
    pub data: RespData,
    pub echo: String,
}

// impl ApiResp {
//     pub fn get_date<D>(&self) -> Option<D> {
//         match self.data {
//             RespData::MessageId(d) => Some(d),
//             _ => None,
//         }
//     }
// }

/// Onebot Api 响应 data 字段
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum RespData {
    None,
    MessageId(MessageId),
    Message(Message),
    Messages(Messages),
    LoginInfo(LoginInfo),
    StrangerInfo(StrangerInfo),
    FriendList(Vec<FriendListItem>),
    GroupInfo(GroupInfo),
    GroupList(Vec<GroupListItem>),
    GroupMemberInfo(GroupMemberInfo),
    GroupMemberList(Vec<GroupMember>),
    GroupHonorInfo(GroupHonorInfo),
    Cookies(Cookies),
    ScrfToken(ScrfToken),
    Credentials(Credentials),
    File(File),
    SendCheck(SendCheck),
    Status(crate::event::Status),
    VersionInfo(VersionInfo),
}

/// message_id 响应数据
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MessageId {
    pub message_id: i32,
}

/// get_msg 响应数据
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub time: i32,
    pub message_type: String,
    pub message_id: i32,
    pub real_id: i32,
    pub sender: Sender,
    pub message: Vec<crate::message::Message>,
}

/// get_forward_msg 响应数据
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Messages {
    pub message: Vec<crate::message::Message>,
}

/// get_login_info 响应数据
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoginInfo {
    #[serde(deserialize_with = "id_deserializer")]
    pub user_id: String,
    pub nickname: String,
}

/// get_stranger_info 响应数据
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StrangerInfo {
    #[serde(deserialize_with = "id_deserializer")]
    pub user_id: String,
    pub nickname: String,
    pub sex: String,
    pub age: i32,
}

/// get_group_info 响应数据
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GroupInfo {
    #[serde(deserialize_with = "id_deserializer")]
    pub groupp_id: String,
    pub group_name: String,
    pub member_count: i32,
    pub max_member_count: i32,
}

/// get_group_member_info 响应数据
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GroupMemberInfo {
    #[serde(deserialize_with = "id_deserializer")]
    pub groupp_id: String,
    #[serde(deserialize_with = "id_deserializer")]
    pub user_id: String,
    pub nickname: String,
    pub card: String,
    pub sex: String,
    pub age: i32,
    pub area: String,
    pub join_time: i32,
    pub last_sent_time: i32,
    pub level: String,
    pub role: String,
    pub unfriendly: bool,
    pub title: String,
    pub title_expire_time: i32,
    pub card_changeable: bool,
}

/// get_group_honor_info 响应数据
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GroupHonorInfo {
    #[serde(deserialize_with = "id_deserializer")]
    pub group_id: String,
    pub current_talkative: Option<CurrentTalkative>,
    pub talkative_list: Option<Vec<HonorItem>>,
    pub performer_list: Option<Vec<HonorItem>>,
    pub legend_list: Option<Vec<HonorItem>>,
    pub strong_newbie_list: Option<Vec<HonorItem>>,
    pub emotion_list: Option<Vec<HonorItem>>,
}

/// get_cookies 响应数据
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Cookies {
    pub cookies: String,
}

/// get_csrf_token 响应数据
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScrfToken {
    pub token: i32,
}

/// get_credentials 响应数据
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Credentials {
    pub cookies: String,
    pub token: i32,
}

/// get_recode && get_image 响应数据
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct File {
    pub file: String,
}

/// can_send_image && can_send_record 响应数据
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SendCheck {
    pub yes: bool,
}

/// get_version_info 响应数据
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VersionInfo {
    pub app_name: String,
    pub app_version: String,
    pub protocol_version: String,
}

/// get_friend_list 响应数组成员
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FriendListItem {
    #[serde(deserialize_with = "id_deserializer")]
    pub user_id: String,
    pub nickname: String,
    pub remark: String,
}

/// get_group_list 响应数组成员
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GroupListItem {
    #[serde(deserialize_with = "id_deserializer")]
    pub group_id: String,
    pub group_name: String,
    pub member_count: i32,
    pub max_member_count: i32,
}

/// get_group_member_list 响应数组成员
#[derive(Debug, Serialize, Deserialize, Clone)] // need check
pub struct GroupMember {
    #[serde(deserialize_with = "id_deserializer")]
    pub group_id: String,
    #[serde(deserialize_with = "id_deserializer")]
    pub user_id: String,
    pub nickname: String,
    pub card: String,
    pub sex: String,
    pub age: i32,
    pub join_time: i32,
    pub last_sent_time: i32,
    pub level: String,
    pub role: String,
    pub unfriendly: bool,
    pub card_changeable: bool,
}

/// get_group_honor_info 相关
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CurrentTalkative {
    #[serde(deserialize_with = "id_deserializer")]
    pub user_id: String,
    pub nickname: String,
    pub avatar: String,
    pub day_count: i32,
}

/// get_group_honor_info 相关
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HonorItem {
    #[serde(deserialize_with = "id_deserializer")]
    pub user_id: String,
    pub nickname: String,
    pub avatar: String,
    pub description: String,
}

/// Onebot Api 响应 sender 字段
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Sender {
    Group(crate::event::GroupSender),
    Private(crate::event::PrivateSender),
}
