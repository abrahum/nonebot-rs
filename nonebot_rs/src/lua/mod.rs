// minimal
use crate::event::SelfId;
use crate::event::{Event, MessageEvent};
use crate::log::{colored::*, event, Level};
use crate::message::Message;
use mlua::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct LuaPlugin {
    bot_getter: Option<crate::BotGettter>,
    scripts: HashMap<String, String>,
}

impl LuaPlugin {
    pub fn new(scripts: HashMap<String, String>) -> Self {
        LuaPlugin {
            bot_getter: None,
            scripts: scripts,
        }
    }

    pub fn run_lua_scripts(&mut self, event: MessageEvent) {
        let bots = self.bot_getter.clone().unwrap().borrow().clone();
        if let Some(bot) = bots.get(&event.get_self_id()) {
            event!(Level::DEBUG, "[{}] Running Lua-Script", bot.bot_id.red());
            for (script_name, script_path) in &self.scripts {
                run_lua_script(&script_name, &script_path, &event, &bot);
            }
            event!(Level::DEBUG, "[{}] Finish Lua-Script", bot.bot_id.red());
        }
    }

    async fn event_recv(mut self, mut event_receiver: crate::EventReceiver) {
        while let Ok(event) = event_receiver.recv().await {
            match event {
                Event::Message(m) => {
                    self.run_lua_scripts(m);
                }

                _ => {}
            }
        }
    }
}

fn run_lua_script(script_name: &str, script_path: &str, event: &MessageEvent, bot: &crate::Bot) {
    let path = std::path::PathBuf::from(&script_path);
    match std::fs::read_to_string(&path) {
        Ok(s) => {
            let bot = bot.clone();
            let event = event.clone();
            let lua = Lua::new();
            lua.globals()
                .set("Message", event.get_raw_message())
                .unwrap();
            lua.load(&s).exec().unwrap();
            let r_msg = lua.globals().get("Rmessage");
            match r_msg {
                Ok(r_msg) => {
                    event!(
                        Level::INFO,
                        "[{}] Matched Lua-Script {}",
                        bot.bot_id.red(),
                        script_name.blue()
                    );
                    tokio::spawn(async move {
                        bot.send_by_message_event(&event, vec![Message::text(r_msg)])
                            .await
                    });
                }
                _ => {}
            }
        }
        Err(e) => event!(Level::WARN, "Open Lua File {} Failed：{}", script_name, e),
    }
}

impl crate::Plugin for LuaPlugin {
    fn run(&self, event_receiver: crate::EventReceiver, bot_getter: crate::BotGettter) {
        let mut l = self.clone();
        l.bot_getter = Some(bot_getter.clone());
        tokio::spawn(l.event_recv(event_receiver));
    }
}
