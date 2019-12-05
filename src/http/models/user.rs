use crate::Snowflake;

#[object(client)]
pub struct ModifyBot {
    pub username: String,
    //pub avatar: Option<???>
}

#[object(client)]
pub struct Recipient {
    pub recipient_id: Snowflake
}