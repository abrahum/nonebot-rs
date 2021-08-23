use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "type", content = "data")]
pub enum Message {
    #[serde(rename = "text")]
    Text(TextMessage), // 纯文本消息
    #[serde(rename = "face")]
    Face(FaceMessage),
    #[serde(rename = "image")]
    Image(ImageMessage),
    #[serde(rename = "record")]
    Record(RecordMessage),
    #[serde(rename = "video")]
    Video(VideoMessage),
    #[serde(rename = "at")]
    At(AtMessage),
    #[serde(rename = "rps")]
    Rps,
    #[serde(rename = "dice")]
    Dice,
    #[serde(rename = "shake")]
    Shake,
    #[serde(rename = "poke")]
    Poke(PokeMessage),
    #[serde(rename = "anonymous")]
    Anonymous,
    #[serde(rename = "share")]
    Share(ShareMessage),
    #[serde(rename = "contact")]
    Contact(ContactMessage),
    #[serde(rename = "location")]
    Lacation(LocationMessage),
    #[serde(rename = "music")]
    Music(MusicMessage),
    #[serde(rename = "reply")]
    Reply(ReplyMessage),
    #[serde(rename = "forward")]
    Forward(ForwardMessage),
    #[serde(rename = "node")]
    Node(NodeMessage),
    #[serde(rename = "xml")]
    Xml(XmlMessage),
    #[serde(rename = "json")]
    Json(JsonMessage),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TextMessage {
    pub text: String, // 文本内容
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FaceMessage {
    pub id: String, // QQ 表情 ID
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ImageMessage {
    pub file: String, // 图片文件名
    #[serde(rename = "type")]
    pub type_: Option<String>, // 图片类型 flash 闪照
    pub url: Option<String>, // 图片 URL
    pub cache: Option<u8>, // 是否使用缓存文件 1|0
    pub proxy: Option<u8>, // 是否使用代理 1|0
    pub timeout: Option<i64>, // 网络文件下载超时 单位秒
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RecordMessage {
    pub file: String,         // 语音文件名
    pub magic: Option<u8>,    // 是否变声 1|0
    pub url: Option<String>,  // 语音 URL
    pub cache: Option<u8>,    // 是否使用缓存文件 1|0
    pub proxy: Option<u8>,    // 是否使用代理 1|0
    pub timeout: Option<i64>, // 网络文件下载超时 单位秒
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct VideoMessage {
    pub file: String,         // 视频文件名
    pub url: Option<String>,  // 视频 URL
    pub cache: Option<u8>,    // 是否使用缓存文件 1|0
    pub proxy: Option<u8>,    // 是否使用代理 1|0
    pub timeout: Option<i64>, // 网络文件下载超时 单位秒
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AtMessage {
    pub qq: String, // @QQ ID all 表示全体
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PokeMessage {
    #[serde(rename = "type")]
    pub type_: String, // 类型
    pub id: String,
    pub name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ShareMessage {
    pub url: String,             // URL
    pub title: String,           // 标题
    pub content: Option<String>, // 内容描述
    pub image: Option<String>,   // 图片 URl
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ContactMessage {
    #[serde(rename = "type")]
    pub type_: String, // 类型 qq|group
    pub id: String, // QQ号|群号
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LocationMessage {
    pub lat: String,             // 纬度
    pub lon: String,             // 经度
    pub title: Option<String>,   // 标题
    pub content: Option<String>, // 内容描述
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MusicMessage {
    #[serde(rename = "type")]
    pub type_: String, // 类型 qq|163|xm|custom
    pub id: Option<String>,      // 歌曲 ID
    pub url: Option<String>,     // 点击后跳转 URL
    pub audio: Option<String>,   // 歌曲 URL
    pub title: Option<String>,   // 标题
    pub content: Option<String>, // 内容描述
    pub image: Option<String>,   // 图片 URl
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ReplyMessage {
    pub id: String, // 回复的消息 ID
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ForwardMessage {
    pub id: String, // 合并转发 ID
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct NodeMessage {
    pub id: Option<String>,            // 转发的消息 ID
    pub user_id: Option<String>,       // 发送者 QQ 号
    pub nickname: Option<String>,      // 发送者昵称
    pub content: Option<Vec<Message>>, // 消息内容
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct XmlMessage {
    pub data: String, // 合并转发 ID
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct JsonMessage {
    pub data: String, // 合并转发 ID
}
