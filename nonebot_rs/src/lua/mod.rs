// minimal
use crate::event::MessageEvent;
use crate::log::{colored::*, event, Level};
use crate::message::Message;
use mlua::prelude::*;
use std::collections::HashMap;

pub fn just_run(
    lua_scripts: &Option<HashMap<String, String>>,
    event: MessageEvent,
    bot: crate::Bot,
) {
    if let Some(lua_scripts) = lua_scripts {
        for (script_name, script) in lua_scripts {
            event!(
                Level::INFO,
                "Running Lua-Script {} for Bot {}",
                script_name.blue(),
                bot.bot_id.red()
            );

            let path = std::path::PathBuf::from(&script);
            match std::fs::read_to_string(&path) {
                Ok(s) => {
                    let lua = Lua::new();
                    lua.globals()
                        .set("Message", event.get_raw_message())
                        .unwrap();
                    lua.load(&s).exec().unwrap();
                    let r_msg: String = lua.globals().get("Rmessage").unwrap();
                    let movebot = bot.clone();
                    match &event {
                        MessageEvent::Private(p) => {
                            let p = p.clone();
                            tokio::spawn(async move {
                                movebot
                                    .send_private_msg(&p.user_id, vec![Message::text(r_msg)])
                                    .await
                            });
                        }
                        MessageEvent::Group(g) => {
                            let g = g.clone();
                            tokio::spawn(async move {
                                movebot
                                    .send_group_msg(&g.group_id, vec![Message::text(r_msg)])
                                    .await
                            });
                        }
                    }
                    event!(
                        Level::INFO,
                        "Finish Lua-Script {} for Bot {}",
                        script_name.blue(),
                        bot.bot_id.red()
                    );
                }
                Err(e) => event!(Level::WARN, "Open Lua File {} Failed：{}", script, e),
            }
        }
    }
}
