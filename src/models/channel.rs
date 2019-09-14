
#[object(both)]
pub struct Channel {

}

#[object(server)]
pub struct ChannelMention {
    pub id: u64,
    pub guild_id: u64,
    pub _type: ChannelType,
    pub name: String
}

#[convert(u8)]
pub enum ChannelType {
    GuildText = 0,
    DM = 1,
    GuildVoice = 2,
    GroupDM = 3,
    GuildCategory = 4,
    GuildNews = 5,
    GuildStore = 6
}