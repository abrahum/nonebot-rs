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
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum RespData {
    None,
    MessageId {
        message_id: i32,
    },
    Message {
        time: i32,
        message_type: String,
        message_id: i32,
        real_id: i32,
        sender: Sender,
        message: Vec<crate::message::Message>,
    },
    Messages {
        message: Vec<crate::message::Message>,
    },
    LoginInfo {
        user_id: i64,
        nickname: String,
    },
    StrangerInfo {
        user_id: i64,
        nickname: String,
        sex: String,
        age: i32,
    },
    FriendList(Vec<FriendListItem>),
    GroupInfo {
        groupp_id: i64,
        group_name: String,
        member_count: i32,
        max_member_count: i32,
    },
    GroupList(Vec<GroupListItem>),
    GroupMemberInfo {
        groupp_id: i64,
        user_id: i64,
        nickname: String,
        card: String,
        sex: String,
        age: i32,
        area: String,
        join_time: i32,
        last_sent_time: i32,
        level: String,
        role: String,
        unfriendly: bool,
        title: String,
        title_expire_time: i32,
        card_changeable: bool,
    },
    GroupMemberList(Vec<GroupMember>),
    GroupHonorInfo {
        group_id: i64,
        current_talkative: Option<CurrentTalkative>,
        talkative_list: Option<Vec<HonorItem>>,
        performer_list: Option<Vec<HonorItem>>,
        legend_list: Option<Vec<HonorItem>>,
        strong_newbie_list: Option<Vec<HonorItem>>,
        emotion_list: Option<Vec<HonorItem>>,
    },
    Cookies {
        cookies: String,
    },
    ScrfToken {
        token: i32,
    },
    Credentials {
        cookies: String,
        token: i32,
    },
    File {
        file: String,
    },
    SendCheck {
        yes: bool,
    },
    Status(crate::event::Status),
    VersionInfo {
        app_name: String,
        app_version: String,
        protocol_version: String,
    },
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
