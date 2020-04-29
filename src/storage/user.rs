use std::collections::HashMap;
use crate::Snowflake;
use crate::gateway::User;
use crate::storage::{Stored, Storage};

#[derive(Default)]
pub struct UserStorage {
    users: HashMap<Snowflake, User>
}

impl Stored for User {
    type Storage = UserStorage;
}

impl Storage for UserStorage {
    type Key = Snowflake;
    type Stored = User;

    fn get(&self, id: &Self::Key) -> &Self::Stored {
        self.find(id).unwrap()
    }

    fn find(&self, id: &Self::Key) -> Option<&Self::Stored> {
        self.users.get(&id)
    }

    fn insert(&mut self, key: &Self::Key, val: &Self::Stored) {
        self.users.insert(*key, (*val).clone());
    }
}

impl UserStorage {
    fn find_by<P>(&self, mut filter: P) -> Vec<&User>
        where P: FnMut(&User) -> bool {
        self.users.values()
            .filter(|u| filter(u))
            .collect()
    }

    fn find_one_by<P>(&self, mut filter: P) -> Option<&User>
        where P: FnMut(&User) -> bool {
        for user in self.users.values() {
            if filter(user) {
                return Some(user);
            }
        }

        None
    }
}