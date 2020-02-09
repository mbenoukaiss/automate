use chrono::NaiveDateTime;

#[derive(Debug)]
pub struct Bucket {
    id: String,
    limit: u16,
    remaining: u16,
    reset: NaiveDateTime
}

impl Bucket {
    pub fn new(id: String, limit: u16, remaining: u16, reset: NaiveDateTime) -> Bucket {
        Bucket {
            id,
            limit,
            remaining,
            reset,
        }
    }
}