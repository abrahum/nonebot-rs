// pub fn register_plugin(nb: crate::Nonebot) {}

pub trait Plugin: std::fmt::Debug {
    fn run(&self, event_receiver: crate::EventReceiver, bot_getter: crate::BotGettter);
}
