use std::collections::HashMap;
use crate::Snowflake;
use crate::gateway::User;
use crate::storage::{Stored, Storage};

#[derive(Default, Debug, Clone)]
pub struct UserStorage {
    users: HashMap<Snowflake, User>
}

impl Storage for UserStorage {}

impl Stored for User {
    type Storage = UserStorage;
}

impl UserStorage {
    pub fn all(&self) -> Vec<&User> {
        self.users.values().collect()
    }

    #[inline]
    pub fn get(&self, id: Snowflake) -> &User {
        self.get_opt(id).unwrap()
    }

    pub fn get_opt(&self, id: Snowflake) -> Option<&User> {
        self.users.get(&id)
    }

    pub(crate) fn get_mut(&mut self, id: Snowflake) -> Option<&mut User> {
        self.users.get_mut(&id)
    }

    pub(crate) fn insert(&mut self, user: User) {
        self.users.insert(user.id, user);
    }
}