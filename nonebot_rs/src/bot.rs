use crate::api_resp::{self, RespData};
use crate::event::MessageEvent;
use crate::{api, config, message, utils, ApiChannelItem, ApiResp};
use colored::*;
use tokio::sync::{mpsc, watch};
use tracing::{event, Level};

/// Bot
#[derive(Debug, Clone)]
pub struct Bot {
    /// bot id
    pub bot_id: String,
    /// connect timestamp
    pub connect_time: i64,
    // Bot Config
    pub config: config::BotConfig,
    /// 暂存调用 Bot api
    pub api_sender: mpsc::Sender<ApiChannelItem>,
    /// 暂存 ApiResp Receiver
    pub api_resp_watcher: watch::Receiver<ApiResp>,
}

macro_rules! no_resp_api {
    ($fn_name: ident, $struct_name: tt, $param: ident: $param_type: ty) => {
        pub async fn $fn_name(&self, $param: $param_type) {
            self.call_api(api::Api::$fn_name(api::$struct_name { $param: $param }))
                .await;
        }
    };
    ($fn_name: ident, $struct_name: tt, $($param: ident: $param_type: ty),*) => {
        pub async fn $fn_name(&self, $($param: $param_type,)*) {
            self.call_api(api::Api::$fn_name(api::$struct_name {
                $($param: $param,)*
            })).await;
        }
    };
}

macro_rules! resp_api {
    ($fn_name: ident,$resp_data: tt, $resp_data_type: ty) => {
        pub async fn $fn_name(&self) -> Option<$resp_data_type> {
            let resp = self.call_api_resp(api::Api::$fn_name()).await;
            if let RespData::$resp_data(d) = resp.unwrap().data {
                Some(d)
            } else {
                None
            }
        }
    };
    ($fn_name: ident, $struct_name: tt, $resp_data: tt, $resp_data_type: ty, $param: ident: $param_type: ty) => {
        pub async fn $fn_name(&self, $param: $param_type) -> Option<$resp_data_type> {
            let resp = self
                .call_api_resp(api::Api::$fn_name(api::$struct_name { $param: $param }))
                .await;
            if let RespData::$resp_data(d) = resp.unwrap().data {
                Some(d)
            } else {
                None
            }
        }
    };
    ($fn_name: ident, $struct_name: tt, $resp_data: tt, $resp_data_type: ty, $($param: ident: $param_type: ty),*) => {
        pub async fn $fn_name(&self, $($param: $param_type,)*) -> Option<$resp_data_type> {
            let resp = self
                .call_api_resp(api::Api::$fn_name(api::$struct_name {
                    $($param: $param,)*
                }))
                .await;
            if let RespData::$resp_data(d) = resp.unwrap().data {
                Some(d)
            } else {
                None
            }
        }
    };
}

impl Bot {
    pub fn new(
        bot_id: String,
        config: config::BotConfig,
        api_sender: mpsc::Sender<ApiChannelItem>,
        api_resp_watcher: watch::Receiver<ApiResp>,
    ) -> Self {
        Bot {
            bot_id: bot_id,
            connect_time: crate::utils::timestamp(),
            config: config,
            api_sender: api_sender,
            api_resp_watcher: api_resp_watcher,
        }
    }

    /// Send Group Msg
    pub async fn send_group_msg(&self, group_id: &str, msg: Vec<message::Message>) {
        self.api_sender
            .send(ApiChannelItem::Api(crate::api::Api::send_group_msg(
                crate::api::SendGroupMsg {
                    group_id: group_id.to_string(),
                    message: msg.clone(),
                    auto_escape: false,
                },
            )))
            .await
            .unwrap();
        event!(
            Level::INFO,
            "Bot [{}] Send {:?} to Group ({})",
            self.config.bot_id.red(),
            msg,
            group_id.to_string().magenta()
        );
    }

    /// Send Private Msg
    pub async fn send_private_msg(&self, user_id: &str, msg: Vec<message::Message>) {
        self.api_sender
            .send(ApiChannelItem::Api(crate::api::Api::send_private_msg(
                crate::api::SendPrivateMsg {
                    user_id: user_id.to_string(),
                    message: msg.clone(),
                    auto_escape: false,
                },
            )))
            .await
            .unwrap();
        event!(
            Level::INFO,
            "Bot [{}] Send {:?} to Friend ({})",
            self.config.bot_id.red(),
            msg,
            user_id.to_string().green()
        );
    }

    /// 根据 MessageEvent 类型发送私聊消息或群消息
    pub async fn send_by_message_event(&self, event: &MessageEvent, msg: Vec<message::Message>) {
        match event {
            MessageEvent::Private(p) => self.send_private_msg(&p.user_id, msg).await,
            MessageEvent::Group(g) => self.send_group_msg(&g.group_id, msg).await,
        }
    }

    /// 请求 Onebot Api，不等待 Onebot 返回
    pub async fn call_api(&self, api: api::Api) {
        self.api_sender
            .send(ApiChannelItem::Api(api.clone()))
            .await
            .unwrap();
        event!(
            Level::INFO,
            "Bot [{}] Calling Api {:?}",
            self.config.bot_id.red(),
            api
        );
    }

    /// 请求 Onebot Api，等待 Onebot 返回项（30s 后 timeout 返回 None）
    pub async fn call_api_resp(&self, api: api::Api) -> Option<api_resp::ApiResp> {
        let echo = api.get_echo();
        self.api_sender
            .send(ApiChannelItem::Api(api.clone()))
            .await
            .unwrap();
        event!(
            Level::INFO,
            "Bot [{}] Calling Api {:?}",
            self.config.bot_id.red(),
            api
        );
        let time = utils::timestamp();
        let mut watcher = self.api_resp_watcher.clone();
        while let Ok(_) = watcher.changed().await {
            let resp = self.api_resp_watcher.borrow().clone();
            if resp.echo == echo {
                return Some(resp);
            }
            if utils::timestamp() > time + 30 {
                return None;
            }
        }
        None
    }

    // pub async fn delete_msg(&self, message_id: i32) {
    //     self.call_api(Api::delete_msg(api::DeleteMsg {
    //         message_id: message_id,
    //     }))
    //     .await;
    // }
    no_resp_api!(delete_msg, DeleteMsg, message_id: i32);
    no_resp_api!(send_like, SendLike, user_id: String, times: u8);
    no_resp_api!(
        set_group_kick,
        SetGroupKick,
        group_id: String,
        user_id: String,
        reject_add_request: bool
    );
    no_resp_api!(
        set_group_ban,
        SetGroupBan,
        group_id: String,
        user_id: String,
        duration: i64
    );
    no_resp_api!(
        set_group_anonymous_ban,
        SetGroupAnonymousBan,
        group_id: String,
        anonymous: crate::event::Anoymous,
        flag: String,
        duration: i64
    );
    no_resp_api!(
        set_group_whole_ban,
        SetGroupWholeBan,
        group_id: String,
        enable: bool
    );
    no_resp_api!(
        set_group_admin,
        SetGroupAdmin,
        group_id: String,
        user_id: String,
        enable: bool
    );
    no_resp_api!(
        set_group_anonymous,
        SetGroupAnonymous,
        group_id: String,
        enable: bool
    );
    no_resp_api!(
        set_group_card,
        SetGroupCard,
        group_id: String,
        user_id: String,
        card: String
    );
    no_resp_api!(
        set_group_name,
        SetGroupName,
        group_id: String,
        group_name: String
    );
    no_resp_api!(
        set_group_leave,
        SetGroupLeave,
        group_id: String,
        is_dismiss: bool
    );
    no_resp_api!(
        set_group_special_title,
        SetGroupSpecialTitle,
        group_id: String,
        user_id: String,
        special_title: String,
        duration: i64
    );
    no_resp_api!(
        set_friend_add_request,
        SetFriendAddRequest,
        flag: String,
        approve: bool,
        remark: String
    );
    no_resp_api!(
        set_group_add_request,
        SetGroupAddRequest,
        flag: String,
        sub_type: String,
        approve: bool,
        reason: String
    );
    no_resp_api!(set_restart, SetRestart, delay: i64);

    // 获取消息
    // pub async fn get_msg(&self, message_id: i32) -> Option<api_resp::Message> {
    //     let resp = self
    //         .call_api_resp(Api::get_msg(api::GetMsg {
    //             message_id: message_id,
    //         }))
    //         .await;
    //     if let RespData::Message(m) = resp.unwrap().data {
    //         Some(m)
    //     } else {
    //         None
    //     }
    // }
    resp_api!(
        send_msg,
        SendMsg,
        MessageId,
        api_resp::MessageId,
        message_type: Option<String>,
        user_id: Option<String>,
        group_id: Option<String>,
        message: Vec<crate::Message>,
        auto_escape: bool
    );
    resp_api!(get_msg, GetMsg, Message, api_resp::Message, message_id: i32);
    resp_api!(
        get_forward_msg,
        GetForwardMsg,
        Message,
        api_resp::Message,
        id: String
    );
    resp_api!(get_login_info, LoginInfo, api_resp::LoginInfo);
    resp_api!(
        get_stranger_info,
        GetStrangerInfo,
        StrangerInfo,
        api_resp::StrangerInfo,
        user_id: String,
        no_cache: bool
    );
    resp_api!(get_friend_list, FriendList, Vec<api_resp::FriendListItem>);
    resp_api!(
        get_group_info,
        GetGroupInfo,
        GroupInfo,
        api_resp::GroupInfo,
        group_id: String,
        no_cache: bool
    );
    resp_api!(get_group_list, GroupList, Vec<api_resp::GroupListItem>);
    resp_api!(
        get_group_member_info,
        GetGroupMemberInfo,
        GroupMemberInfo,
        api_resp::GroupMemberInfo,
        group_id: String,
        user_id: String,
        no_cache: bool
    );
    resp_api!(
        get_group_member_list,
        GetGroupMemberList,
        GroupMemberList,
        Vec<api_resp::GroupMember>,
        group_id: String
    );
    resp_api!(
        get_group_honor_info,
        GetGroupHonorInfo,
        GroupHonorInfo,
        api_resp::GroupHonorInfo,
        group_id: String,
        type_: String
    );
    resp_api!(
        get_cookies,
        GetCookies,
        Cookies,
        api_resp::Cookies,
        domain: String
    );
    resp_api!(get_csrf_token, ScrfToken, api_resp::ScrfToken);
    resp_api!(
        get_credentials,
        GetCookies,
        Credentials,
        api_resp::Credentials,
        domain: String
    );
    resp_api!(
        get_record,
        GetRecord,
        File,
        api_resp::File,
        file: String,
        out_format: String
    );
    resp_api!(get_image, GetImage, File, api_resp::File, file: String);
    resp_api!(can_send_record, SendCheck, api_resp::SendCheck);
    resp_api!(can_send_image, SendCheck, api_resp::SendCheck);
    resp_api!(get_status, Status, crate::event::Status);
    resp_api!(get_version_info, VersionInfo, api_resp::VersionInfo);
}
