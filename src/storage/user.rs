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

    pub fn get(&self, id: Snowflake) -> &User {
        self.find(id).unwrap()
    }

    pub fn find(&self, id: Snowflake) -> Option<&User> {
        self.users.get(&id)
    }

    pub fn find_by<P>(&self, mut filter: P) -> Vec<&User>
        where P: FnMut(&User) -> bool {
        self.users.values()
            .filter(|u| filter(u))
            .collect()
    }

    pub fn find_one_by<P>(&self, mut filter: P) -> Option<&User>
        where P: FnMut(&User) -> bool {
        for user in self.users.values() {
            if filter(user) {
                return Some(user);
            }
        }

        None
    }

    pub fn insert(&mut self, user: &User) {
        self.users.insert(user.id, Clone::clone(user));
    }
}