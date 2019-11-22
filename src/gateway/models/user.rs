use crate::json::Nullable;
use crate::gateway::PartialGuildMember;

/// Users in Discord are generally considered the
/// base entity. Users can spawn across the entire
/// platform, be members of guilds, participate in
/// text and voice chat, and much more. Users are
/// separated by a distinction of "bot" vs "normal."
/// Although they are similar, bot users are
/// automated users that are "owned" by another user.
/// Unlike normal users, bot users do not have a
/// limitation on the number of Guilds they can be
/// a part of.
///
/// More information on [Discord's documentation](https://discordapp.com/developers/docs/resources/user#user-object)
#[object(both)]
pub struct User {
    pub id: u64,
    pub username: String,
    pub discriminator: String,
    pub avatar: Nullable<String>,
    pub bot: Option<bool>,
    pub mfa_enabled: Option<bool>,
    pub locale: Option<String>,
    pub verified: Option<bool>,
    pub email: Option<Nullable<String>>,
    pub flags: Option<i32>,
    pub premium_type: Option<i32>
}

/// A [User] object with all fields optional
/// except for  ̀id`.
///
/// More information on [Discord's documentation](https://discordapp.com/developers/docs/topics/gateway#presence-update)
#[object(server)]
pub struct PartialUser {
    pub id: u64,
    pub username: Option<String>,
    pub discriminator: Option<String>,
    pub avatar: Option<Nullable<String>>,
    pub bot: Option<bool>,
    pub mfa_enabled: Option<bool>,
    pub locale: Option<String>,
    pub verified: Option<bool>,
    pub email: Option<String>,
    pub flags: Option<i32>,
    pub premium_type: Option<i32>
}

#[object(server)]
pub struct MentionnedUser {
    pub id: u64,
    pub member: PartialGuildMember,
    pub username: String,
    pub discriminator: String,
    pub avatar: Nullable<String>,
    pub bot: Option<bool>,
    pub mfa_enabled: Option<bool>,
    pub locale: Option<String>,
    pub verified: Option<bool>,
    pub email: Option<Nullable<String>>,
    pub flags: Option<i32>,
    pub premium_type: Option<i32>
}