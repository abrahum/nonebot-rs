use serde::{Deserialize, Serialize};

/// Onbot 协议规定的消息接口
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "type", content = "data")]
pub enum Message {
    /// 纯文本
    #[serde(rename = "text")]
    Text {
        /// 纯文本内容
        text: String,
    },

    /// QQ 表情
    #[serde(rename = "face")]
    Face {
        /// QQ 表情 ID
        id: String,
    },

    /// 图片
    #[serde(rename = "image")]
    Image {
        /// 图片文件名
        file: String,
        /// 图片类型 flash 闪照
        #[serde(rename = "type")]
        type_: Option<String>,
        /// 图片 URL
        url: Option<String>,
        /// 是否使用缓存文件 1|0
        cache: Option<u8>,
        /// 是否使用代理 1|0
        proxy: Option<u8>,
        /// 网络文件下载超时 单位秒
        timeout: Option<i64>,
    },

    /// 语音
    #[serde(rename = "record")]
    Record {
        /// 语音文件名
        file: String,
        /// 是否变声 1|0
        magic: Option<u8>,
        /// 语音 URL    
        url: Option<String>,
        /// 是否使用缓存文件 1|0
        cache: Option<u8>,
        /// 是否使用代理 1|0
        proxy: Option<u8>,
        /// 网络文件下载超时 单位秒
        timeout: Option<i64>,
    },

    /// 短视频
    #[serde(rename = "video")]
    Video {
        /// 视频文件名
        file: String,
        /// 视频 URL
        url: Option<String>,
        /// 是否使用缓存文件 1|0
        cache: Option<u8>,
        /// 是否使用代理 1|0
        proxy: Option<u8>,
        /// 网络文件下载超时 单位秒
        timeout: Option<i64>,
    },

    /// @某人
    #[serde(rename = "at")]
    At {
        /// @QQ ID all 表示全体
        qq: String,
    },

    /// 猜拳魔法表情
    #[serde(rename = "rps")]
    Rps,

    /// 掷骰子魔法表情
    #[serde(rename = "dice")]
    Dice,

    /// 窗口抖动（戳一戳）
    #[serde(rename = "shake")]
    Shake,

    /// 戳一戳
    #[serde(rename = "poke")]
    Poke {
        /// 类型
        #[serde(rename = "type")]
        type_: String,
        /// ID
        id: String,
        /// 表情名
        name: Option<String>,
    },

    /// 匿名发消息
    #[serde(rename = "anonymous")]
    Anonymous,

    /// 链接分享
    #[serde(rename = "share")]
    Share {
        /// URL
        url: String,
        /// 标题
        title: String,
        /// 内容描述
        content: Option<String>,
        /// 图片 URl
        image: Option<String>,
    },

    /// 推荐好友|群
    #[serde(rename = "contact")]
    Contact {
        /// 类型 qq|group
        #[serde(rename = "type")]
        type_: String,
        /// QQ号|群号
        id: String,
    },

    /// 位置
    #[serde(rename = "location")]
    Lacation {
        /// 纬度
        lat: String,
        /// 经度           
        lon: String,
        /// 标题  
        title: Option<String>,
        /// 内容描述
        content: Option<String>,
    },

    /// 音乐分享
    #[serde(rename = "music")]
    Music {
        /// 类型 qq|163|xm|custom
        #[serde(rename = "type")]
        type_: String,
        /// 歌曲 ID
        id: Option<String>,
        /// 点击后跳转 URL
        url: Option<String>,
        /// 歌曲 URL  
        audio: Option<String>,
        /// 标题   
        title: Option<String>,
        /// 内容描述
        content: Option<String>,
        /// 图片 URl
        image: Option<String>,
    },

    /// 回复
    #[serde(rename = "reply")]
    Reply {
        /// 回复的消息 ID
        id: String,
    },

    /// 合并转发
    #[serde(rename = "forward")]
    Forward {
        /// 合并转发 ID
        id: String,
    },

    /// 合并转发节点
    #[serde(rename = "node")]
    Node {
        /// 转发的消息 ID
        id: Option<String>,
        /// 发送者 QQ 号        
        user_id: Option<String>,
        /// 发送者昵称   
        nickname: Option<String>,
        /// 消息内容     
        content: Option<Vec<Message>>,
    },

    /// XML 消息
    #[serde(rename = "xml")]
    Xml {
        /// 合并转发 ID
        data: String,
    },

    /// JSON 消息
    #[serde(rename = "json")]
    Json {
        /// 合并转发 ID
        data: String,
    },
}

impl Message {
    /// 纯文本消息构建函数
    pub fn text(text: &str) -> Message {
        Message::Text {
            text: text.to_string(),
        }
    }

    /// at 消息构建函数
    pub fn at(id: &str) -> Message {
        Message::At { qq: id.to_string() }
    }
}
