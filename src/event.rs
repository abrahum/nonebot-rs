use crate::message::Message;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "post_type")]
pub enum Events {
    #[serde(rename = "message")]
    Message(MessageEvent), // 消息事件
    #[serde(rename = "notice")]
    Notice(NoticeEvent), // 通知事件
    #[serde(rename = "request")]
    Request(RequestEvent), // 请求事件
    #[serde(rename = "meta_event")]
    Meta(MetaEvent), // 元事件
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "message_type")]
pub enum MessageEvent {
    #[serde(rename = "private")]
    Private(PrivateMessageEvent),
    #[serde(rename = "group")]
    Group(GroupMessageEvent),
}

impl MessageEvent {
    #[allow(dead_code)]
    pub fn get_raw_message(&self) -> &str {
        match self {
            MessageEvent::Private(p) => &p.raw_message,
            MessageEvent::Group(g) => &g.raw_message,
        }
    }

    #[allow(dead_code)]
    pub fn get_self_id(&self) -> String {
        match self {
            MessageEvent::Private(p) => p.self_id.to_string(),
            MessageEvent::Group(g) => g.self_id.to_string(),
        }
    }

    #[allow(dead_code)]
    pub fn set_raw_message(&self, new_raw_message: String) -> MessageEvent {
        match self {
            MessageEvent::Private(p) => {
                let mut p = p.clone();
                p.raw_message = new_raw_message;
                MessageEvent::Private(p)
            }
            MessageEvent::Group(g) => {
                let mut g = g.clone();
                g.raw_message = new_raw_message;
                MessageEvent::Group(g)
            }
        }
    }

    #[allow(dead_code)]
    pub fn get_message(&self) -> &Vec<Message> {
        match self {
            MessageEvent::Private(p) => &p.message,
            MessageEvent::Group(g) => &g.message,
        }
    }

    #[allow(dead_code)]
    pub fn get_sender_nickname(&self) -> &str {
        match self {
            MessageEvent::Private(p) => &p.sender.nickname,
            MessageEvent::Group(g) => &g.sender.nickname,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PrivateMessageEvent {
    pub time: i64,             // Event 时间戳
    pub self_id: i64,          // 收到事件的机器人 QQ 号
    pub sub_type: String,      // 消息子类型
    pub message_id: i32,       // 消息 ID
    pub user_id: i64,          // 发送者 ID
    pub message: Vec<Message>, // Array 消息内容
    pub raw_message: String,   // 原生消息内容
    pub font: i32,             // 字体
    pub sender: PrivateSender, // 发送者消息
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PrivateSender {
    pub user_id: i64,     // 发送者 QQ 号
    pub nickname: String, // 昵称
    pub sex: String,      // 性别 male|female|unkown
    pub age: i32,         // 年龄
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GroupMessageEvent {
    pub time: i64,                   // Event 时间戳
    pub self_id: i64,                // 收到事件的机器人 QQ 号
    pub sub_type: String,            // 消息子类型
    pub message_id: i32,             // 消息 ID
    pub group_id: i64,               // 群消息群号
    pub user_id: i64,                // 发送者 ID
    pub anonymous: Option<Anoymous>, // 匿名消息 非匿名消息为空
    pub message: Vec<Message>,       // Array 消息内容
    pub raw_message: String,         // 原生消息内容
    pub font: i32,                   // 字体
    pub sender: GroupSender,         // 发送者消息
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GroupSender {
    pub user_id: i64,     // 发送者 QQ 号
    pub nickname: String, // 昵称
    pub card: String,     // 群名片|备注
    pub sex: String,      // 性别 male|female|unkown
    pub age: i32,         // 年龄
    pub area: String,     // 地区
    pub level: String,    // 成员等级
    pub role: String,     // 角色 owner|admin|member
    pub title: String,    // 专属头衔
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Anoymous {
    pub id: i64,      // 匿名用户 ID
    pub name: String, // 匿名用户名称
    pub flag: String, // 匿名用户 flag
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NoticeEvent {
    pub time: i64,                  // Event 时间戳
    pub self_id: i64,               // 收到事件的机器人 QQ 号
    pub notice_type: String,        // 上报类型
    pub sub_type: Option<String>,   // 事件子类型
    pub group_id: Option<i64>,      // 群消息群号
    pub operator_id: Option<i64>,   // 操作者 QQ 号
    pub user_id: i64,               // 发送者 ID
    pub file: Option<File>,         // 文件信息
    pub duration: Option<i64>,      // 禁言时长，单位秒
    pub message_id: Option<i64>,    // 被撤回的消息 ID
    pub target_id: Option<i64>,     // 目标 QQ 号
    pub honor_type: Option<String>, // 群荣耀类型
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct File {
    pub id: String,   // 文件 ID
    pub name: String, // 文件名
    pub size: i64,    // 文件大小（字节数）
    pub busid: i64,   // 用途未知
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RequestEvent {
    pub time: i64,                // Event 时间戳
    pub self_id: i64,             // 收到事件的机器人 QQ 号
    pub request_type: String,     // 请求类型
    pub user_id: i64,             // 发送请求的 QQ 号
    pub comment: String,          // 验证信息
    pub flag: String,             // 请求 flag
    pub sub_type: Option<String>, // 请求子类型
    pub group_id: Option<i64>,    // 群号
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MetaEvent {
    pub time: i64,                // Event 时间戳
    pub self_id: i64,             // 收到事件的机器人 QQ 号
    pub meta_event_type: String,  // 元事件类型 lifecycle|heartbeat
    pub sub_type: Option<String>, // 事件子类型
    pub status: Option<Status>,   // 状态信息
    pub interval: Option<i64>,    // 下次心跳间隔，单位毫秒
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Status {
    pub online: Option<bool>, // 是否在线，None 表示无法查询
    pub good: bool,           // 运行状态是否符合预期
}
