// pub fn register_plugin(nb: crate::Nonebot) {}

pub mod prelude {
    pub use super::Plugin;
    pub use toml;
}

pub trait Plugin: std::fmt::Debug {
    /// Plugin 启动函数，在 nb 启动时调用一次，不应当阻塞
    fn run(&self, event_receiver: crate::EventReceiver, bot_getter: crate::BotGettter);
    /// Plugin Name 用于注册 Plugin 时标识唯一性
    fn plugin_name(&self) -> &'static str;
    /// Load config
    #[allow(unused_variables)]
    fn load_config(&mut self, config: toml::Value) {}
}
