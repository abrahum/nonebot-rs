// minimal
use mlua::prelude::*;
use nonebot_rs::log::{colored::*, event, Level};
use nonebot_rs::plugin::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;

/// Lua Plugin struct
#[derive(Debug, Clone, Deserialize)]
pub struct LuaPlugin {
    #[serde(skip)]
    bot_getter: Option<nonebot_rs::BotGettter>,
    #[serde(default)]
    #[serde(flatten)]
    scripts: HashMap<String, String>,
    #[serde(default)]
    config: HashMap<String, String>,
}

impl LuaPlugin {
    pub fn new() -> Self {
        LuaPlugin {
            bot_getter: None,
            scripts: HashMap::new(),
            config: HashMap::new(),
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

    async fn event_recv(mut self, mut event_receiver: nonebot_rs::EventReceiver) {
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

fn run_lua_script(
    script_name: &str,
    script_path: &str,
    event: &MessageEvent,
    bot: &nonebot_rs::Bot,
) {
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

impl nonebot_rs::Plugin for LuaPlugin {
    fn run(&self, event_receiver: nonebot_rs::EventReceiver, bot_getter: nonebot_rs::BotGettter) {
        let mut l = self.clone();
        l.bot_getter = Some(bot_getter.clone());
        tokio::spawn(l.event_recv(event_receiver));
    }

    fn plugin_name(&self) -> &'static str {
        "Lua"
    }

    fn load_config(&mut self, config: toml::Value) {
        let luap: LuaPlugin = config.try_into().expect("Lua get error config");
        self.scripts = luap.scripts;
        event!(Level::INFO, "Loaded lua scripts: {:?}", self.scripts);
        self.config = luap.config;
        event!(Level::INFO, "Loaded lua config: {:?}", self.config);
    }
}
