pub struct Bot {
    self_id: String,
}

impl Bot {
    pub fn new(id: u64) -> Self {
        Bot {
            self_id: id.to_string(),
        }
    }
}
