// minimal
use crate::event::MessageEvent;
use crate::log::{colored::*, event, Level};
use crate::message::Message;
use mlua::prelude::*;
use std::collections::HashMap;

pub fn run_lua_scripts(
    lua_scripts: &Option<HashMap<String, String>>,
    event: MessageEvent,
    bot: crate::Bot,
) {
    if let Some(lua_scripts) = lua_scripts {
        event!(Level::DEBUG, "[{}] Running Lua-Script", bot.bot_id.red());
        for (script_name, script_path) in lua_scripts {
            run_lua_script(&script_name, &script_path, &event, &bot);
        }
        event!(Level::DEBUG, "[{}] Finish Lua-Script", bot.bot_id.red());
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
