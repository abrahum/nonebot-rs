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
