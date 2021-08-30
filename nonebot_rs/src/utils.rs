pub fn remove_space(s: &str) -> String {
    let mut rstring = String::from(s);
    let mut chars = s.chars();
    while chars.next() == Some(' ') {
        rstring.remove(0);
    }
    rstring
}

use chrono::Local;

pub fn timestamp() -> i64 {
    let time = Local::now();
    time.timestamp()
}
