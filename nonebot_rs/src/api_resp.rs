use serde::{Deserialize, Serialize};

/// Onebot Api 响应
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MessageId {
    pub message_id: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub time: i32,
    pub message_type: String,
    pub message_id: i32,
    pub real_id: i32,
    pub sender: Sender,
    pub message: Vec<crate::message::Message>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Messages {
    pub message: Vec<crate::message::Message>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoginInfo {
    pub user_id: i64,
    pub nickname: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StrangerInfo {
    pub user_id: i64,
    pub nickname: String,
    pub sex: String,
    pub age: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GroupInfo {
    pub groupp_id: i64,
    pub group_name: String,
    pub member_count: i32,
    pub max_member_count: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GroupMemberInfo {
    pub groupp_id: i64,
    pub user_id: i64,
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GroupHonorInfo {
    pub group_id: i64,
    pub current_talkative: Option<CurrentTalkative>,
    pub talkative_list: Option<Vec<HonorItem>>,
    pub performer_list: Option<Vec<HonorItem>>,
    pub legend_list: Option<Vec<HonorItem>>,
    pub strong_newbie_list: Option<Vec<HonorItem>>,
    pub emotion_list: Option<Vec<HonorItem>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Cookies {
    pub cookies: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScrfToken {
    pub token: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Credentials {
    pub cookies: String,
    pub token: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct File {
    pub file: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SendCheck {
    pub yes: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VersionInfo {
    pub app_name: String,
    pub app_version: String,
    pub protocol_version: String,
}

/// Onebot Api get_friend_list 响应数组成员
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FriendListItem {
    pub user_id: i64,
    pub nickname: String,
    pub remark: String,
}

/// Onebot Api get_group_list 响应数组成员
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GroupListItem {
    pub group_id: i64,
    pub group_name: String,
    pub member_count: i32,
    pub max_member_count: i32,
}

/// Onebot Api get_group_list 响应数组成员
#[derive(Debug, Serialize, Deserialize, Clone)] // need check
pub struct GroupMember {
    pub groupp_id: i64,
    pub user_id: i64,
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CurrentTalkative {
    pub user_id: i64,
    pub nickname: String,
    pub avatar: String,
    pub day_count: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HonorItem {
    pub user_id: i64,
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
