use crate::event::Event;
use async_recursion::async_recursion;
use tokio::sync::broadcast;
use tracing::{event, Level};

/// 去除字符串前方多余空格
#[allow(dead_code)]
pub fn remove_space(s: &str) -> String {
    let mut rstring = String::from(s);
    let mut chars = s.chars();
    while chars.next() == Some(' ') {
        rstring.remove(0);
    }
    rstring
}

use chrono::Local;

#[allow(dead_code)]
pub fn timestamp() -> i64 {
    let time = Local::now();
    time.timestamp()
}

#[async_recursion]
pub async fn send_event(sender: &broadcast::Sender<Event>, e: Event) -> () {
    match sender.send(e) {
        Ok(_) => (),
        Err(broadcast::error::SendError(_)) => {
            event!(Level::ERROR, "EventChannel is full out of cache!");
            std::process::exit(101);
        }
    }
}

use serde::Deserializer;

struct JsonIdVisitor;

impl<'de> serde::de::Visitor<'de> for JsonIdVisitor {
    type Value = String;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a i64 or str containing json data")
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(v.to_string())
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(v.to_string())
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(v.to_string())
    }
}

pub fn id_deserializer<'de, D>(d: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    d.deserialize_any(JsonIdVisitor)
}

struct OptionJsonIdVisitor;

impl<'de> serde::de::Visitor<'de> for OptionJsonIdVisitor {
    type Value = Option<String>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a i64 or str containing json data")
    }

    fn visit_some<D>(self, d: D) -> Result<Option<String>, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        Ok(Some(d.deserialize_any(JsonIdVisitor)?))
    }
}

pub fn option_id_deserializer<'de, D>(d: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    d.deserialize_option(OptionJsonIdVisitor)
}
