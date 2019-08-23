
#[object(server)]
pub struct Guild {

}

#[object(server)]
pub struct UnavailableGuild {
    pub id: String, //TODO: replace with u64 when custom deserializer is available
    pub unavailable: bool
}