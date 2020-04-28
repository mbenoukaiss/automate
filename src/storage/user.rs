use std::collections::HashMap;
use crate::Snowflake;
use crate::gateway::User;
use crate::storage::{Stored, StorageContainer, Storage};

#[derive(Default)]
pub struct UserStorage {
    users: HashMap<Snowflake, User>
}

impl Stored for User {
    type Storage = UserStorage;

    fn read(container: &StorageContainer) -> &Self::Storage {
        &container.user_storage
    }

    fn write(container: &mut StorageContainer) -> &mut Self::Storage {
        &mut container.user_storage
    }
}

impl Storage for UserStorage {
    type Stored = User;

    fn get(&self, id: Snowflake) -> &Self::Stored {
        self.find(id).unwrap()
    }

    fn find(&self, id: Snowflake) -> Option<&Self::Stored> {
        self.users.get(&id)
    }

    fn find_by<P>(&self, mut filter: P) -> Vec<&Self::Stored>
        where P: FnMut(&Self::Stored) -> bool {
        self.users.values()
            .filter(|u| filter(u))
            .collect()
    }

    fn find_one_by<P>(&self, mut filter: P) -> Option<&Self::Stored>
        where P: FnMut(&Self::Stored) -> bool {
        for user in self.users.values() {
            if filter(user) {
                return Some(user);
            }
        }

        None
    }

    fn insert(&mut self, val: Self::Stored) {
        self.users.insert(val.id, val);
    }
}