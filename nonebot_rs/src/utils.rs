pub fn remove_space(s: &str) -> String {
    let mut rstring = String::from(s);
    let mut chars = s.chars();
    while chars.next() == Some(' ') {
        rstring.remove(0);
    }
    rstring
}

use chrono::{Duration, Local};

pub fn timestamp() -> i64 {
    let time = Local::now();
    time.timestamp()
}

pub fn later_timestamp(sec: i64) -> i64 {
    let now = Local::now();
    let dur = Duration::seconds(sec);
    let time = now + dur;
    time.timestamp()
}
