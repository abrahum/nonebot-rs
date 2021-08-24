pub fn remove_space(s: &str) -> String {
    let mut rstring = String::new();
    let mut not_space = false;
    for ch in s.chars() {
        if !not_space && ch == ' ' {
            continue;
        } else if !not_space && ch != ' ' {
            not_space = true;
            rstring.push(ch)
        } else if not_space {
            rstring.push(ch)
        }
    }
    rstring
}
