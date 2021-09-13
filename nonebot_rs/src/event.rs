use crate::message::Message;
use crate::utils::{id_deserializer, option_id_deserializer};
use serde::{Deserialize, Serialize};

/// WebSocket 接受数据枚举 Event || ApiResp
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RecvItem {
    Event(Event),
    ApiResp(crate::api_resp::ApiResp),
}

/// Onebot 事件
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "post_type")]
pub enum Event {
    /// 消息事件
    #[serde(rename = "message")]
    Message(MessageEvent),

    /// 通知事件
    #[serde(rename = "notice")]
    Notice(NoticeEvent),

    /// 请求事件
    #[serde(rename = "request")]
    Request(RequestEvent),

    /// 元事件
    #[serde(rename = "meta_event")]
    Meta(MetaEvent),

    /// Nonebot 内部事件
    #[serde(skip)]
    Nonebot(NbEvent),
}

/// Nonebot Event
#[derive(Debug, Clone)]
pub enum NbEvent {
    BotConnect { bot: crate::Bot },
    BotDisconnect { bot: crate::Bot },
}

/// 消息事件
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "message_type")]
pub enum MessageEvent {
    /// 私聊事件
    #[serde(rename = "private")]
    Private(PrivateMessageEvent),

    /// 群消息事件
    #[serde(rename = "group")]
    Group(GroupMessageEvent),
}

impl MessageEvent {
    /// 消息事件时间戳
    #[allow(dead_code)]
    pub fn get_time(&self) -> i64 {
        match self {
            MessageEvent::Private(p) => p.time,
            MessageEvent::Group(g) => g.time,
        }
    }

    /// 消息事件字符串格式消息
    #[allow(dead_code)]
    pub fn get_raw_message(&self) -> &str {
        match self {
            MessageEvent::Private(p) => &p.raw_message,
            MessageEvent::Group(g) => &g.raw_message,
        }
    }

    /// 消息事件设置字符串格式消息
    #[allow(dead_code)]
    pub fn set_raw_message(&mut self, new_raw_message: String) -> MessageEvent {
        match self {
            MessageEvent::Private(p) => {
                p.raw_message = new_raw_message;
                MessageEvent::Private(p.clone())
            }
            MessageEvent::Group(g) => {
                g.raw_message = new_raw_message;
                MessageEvent::Group(g.clone())
            }
        }
    }

    /// 消息事件数组格式消息
    #[allow(dead_code)]
    pub fn get_message(&self) -> &Vec<Message> {
        match self {
            MessageEvent::Private(p) => &p.message,
            MessageEvent::Group(g) => &g.message,
        }
    }

    /// 消息事件发送者昵称
    #[allow(dead_code)]
    pub fn get_sender_nickname(&self) -> &str {
        match self {
            MessageEvent::Private(p) => &p.sender.nickname,
            MessageEvent::Group(g) => &g.sender.nickname,
        }
    }
}

/// 私聊消息事件
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PrivateMessageEvent {
    /// Event 时间戳
    pub time: i64,
    /// 收到事件的机器人 QQ 号
    #[serde(deserialize_with = "id_deserializer")]
    pub self_id: String,
    /// 消息子类型
    pub sub_type: String,
    /// 消息 ID
    pub message_id: i32,
    /// 发送者 ID
    #[serde(deserialize_with = "id_deserializer")]
    pub user_id: String,
    /// Array 消息内容
    pub message: Vec<Message>,
    /// 原生消息内容
    pub raw_message: String,
    /// 字体
    pub font: i32,
    /// 发送者消息
    pub sender: PrivateSender,
}

/// 私聊消息事件发送者
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PrivateSender {
    /// 发送者 QQ 号
    #[serde(deserialize_with = "id_deserializer")]
    pub user_id: String,
    /// 昵称
    pub nickname: String,
    /// 性别 male|female|unkown
    pub sex: String,
    /// 年龄
    pub age: i32,
}

/// 群消息事件
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GroupMessageEvent {
    /// Event 时间戳
    pub time: i64,
    /// 收到事件的机器人 QQ 号
    #[serde(deserialize_with = "id_deserializer")]
    pub self_id: String,
    /// 消息子类型
    pub sub_type: String,
    /// 消息 ID
    pub message_id: i32,
    /// 群消息群号
    #[serde(deserialize_with = "id_deserializer")]
    pub group_id: String,
    /// 发送者 ID
    #[serde(deserialize_with = "id_deserializer")]
    pub user_id: String,
    /// 匿名消息 非匿名消息为空
    pub anonymous: Option<Anoymous>,
    /// Array 消息内容
    pub message: Vec<Message>,
    /// 原生消息内容
    pub raw_message: String,
    /// 字体
    pub font: i32,
    /// 发送者消息
    pub sender: GroupSender,
}

/// 群消息事件发送者
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GroupSender {
    /// 发送者 QQ 号
    #[serde(deserialize_with = "id_deserializer")]
    pub user_id: String,
    /// 昵称
    pub nickname: String,
    /// 群名片|备注
    pub card: String,
    /// 性别 male|female|unkown
    pub sex: String,
    /// 年龄
    pub age: i32,
    /// 地区
    pub area: String,
    /// 成员等级
    pub level: String,
    /// 角色 owner|admin|member
    pub role: String,
    /// 专属头衔
    pub title: String,
}

/// 消息事件匿名字段
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Anoymous {
    /// 匿名用户 ID
    #[serde(deserialize_with = "id_deserializer")]
    pub id: String,
    /// 匿名用户名称
    pub name: String,
    /// 匿名用户 flag
    pub flag: String,
}

/// 通知事件
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NoticeEvent {
    /// Event 时间戳
    pub time: i64,
    /// 收到事件的机器人 QQ 号
    #[serde(deserialize_with = "id_deserializer")]
    pub self_id: String,
    /// 上报类型
    pub notice_type: String,
    /// 事件子类型
    pub sub_type: Option<String>,
    /// 群消息群号
    #[serde(deserialize_with = "option_id_deserializer")]
    #[serde(default)]
    pub group_id: Option<String>,
    /// 操作者 QQ 号
    #[serde(deserialize_with = "option_id_deserializer")]
    #[serde(default)]
    pub operator_id: Option<String>,
    /// 发送者 ID
    #[serde(deserialize_with = "id_deserializer")]
    pub user_id: String,
    /// 文件信息
    pub file: Option<File>,
    /// 禁言时长，单位秒
    pub duration: Option<i64>,
    /// 被撤回的消息 ID
    pub message_id: Option<i64>,
    /// 目标 QQ 号
    #[serde(deserialize_with = "option_id_deserializer")]
    #[serde(default)]
    pub target_id: Option<String>,
    /// 群荣耀类型
    pub honor_type: Option<String>,
}

/// 通知事件文件字段
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct File {
    /// 文件 ID
    pub id: String,
    /// 文件名
    pub name: String,
    /// 文件大小（字节数）
    pub size: i64,
    /// 用途未知
    pub busid: i64,
}

/// 请求事件
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RequestEvent {
    /// Event 时间戳
    pub time: i64,
    /// 收到事件的机器人 QQ 号
    #[serde(deserialize_with = "id_deserializer")]
    pub self_id: String,
    /// 请求类型
    pub request_type: String,
    /// 发送请求的 QQ 号
    #[serde(deserialize_with = "id_deserializer")]
    pub user_id: String,
    /// 验证信息
    pub comment: String,
    /// 请求 flag
    pub flag: String,
    /// 请求子类型
    pub sub_type: Option<String>,
    /// 群号
    #[serde(deserialize_with = "option_id_deserializer")]
    #[serde(default)]
    pub group_id: Option<String>,
}

/// 元事件
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MetaEvent {
    /// Event 时间戳
    pub time: i64,
    /// 收到事件的机器人 QQ 号
    #[serde(deserialize_with = "id_deserializer")]
    pub self_id: String,
    /// 元事件类型 lifecycle|heartbeat
    pub meta_event_type: String,
    /// 事件子类型
    pub sub_type: Option<String>,
    /// 状态信息
    pub status: Option<Status>,
    /// 下次心跳间隔，单位毫秒
    pub interval: Option<i64>,
}

#[test]
fn de_test() {
    let test_str = "{\"group_id\":101,\"message_id\":111,\"notice_type\":\"group_recall\",\"operator_id\":11,\"post_type\":\"notice\",\"self_id\":11,\"time\":1631193409,\"user_id\":11}\n";
    let _meta: Event = serde_json::from_str(test_str).unwrap();
}

/// 元事件状态字段
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Status {
    /// 是否在线，None 表示无法查询
    pub online: Option<bool>,
    /// 运行状态是否符合预期
    pub good: bool,
}

/// `get_user_id()` trait
pub trait UserId {
    fn get_user_id(&self) -> String;
}

impl UserId for MessageEvent {
    fn get_user_id(&self) -> String {
        match self {
            MessageEvent::Private(p) => p.user_id.to_string(),
            MessageEvent::Group(g) => g.user_id.to_string(),
        }
    }
}

impl UserId for NoticeEvent {
    fn get_user_id(&self) -> String {
        self.user_id.clone()
    }
}

impl UserId for RequestEvent {
    fn get_user_id(&self) -> String {
        self.user_id.clone()
    }
}

/// `get_self_id()` trait
pub trait SelfId {
    fn get_self_id(&self) -> String;
}

impl SelfId for MessageEvent {
    fn get_self_id(&self) -> String {
        match self {
            MessageEvent::Private(p) => p.self_id.clone(),
            MessageEvent::Group(g) => g.self_id.clone(),
        }
    }
}

impl SelfId for RequestEvent {
    fn get_self_id(&self) -> String {
        self.self_id.clone()
    }
}

impl SelfId for NoticeEvent {
    fn get_self_id(&self) -> String {
        self.self_id.clone()
    }
}

impl SelfId for MetaEvent {
    fn get_self_id(&self) -> String {
        self.self_id.clone()
    }
}

impl SelfId for Event {
    fn get_self_id(&self) -> String {
        match self {
            Event::Message(e) => e.get_self_id(),
            Event::Request(e) => e.get_self_id(),
            Event::Notice(e) => e.get_self_id(),
            Event::Meta(e) => e.get_self_id(),
            Event::Nonebot(e) => match e {
                NbEvent::BotConnect { bot } => bot.bot_id.clone(),
                NbEvent::BotDisconnect { bot } => bot.bot_id.clone(),
            },
        }
    }
}
