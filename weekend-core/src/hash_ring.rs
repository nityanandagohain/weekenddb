use std::collections::HashMap;
use crate::server::ServerThread;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::borrow::Borrow;

// Impl of consistent hashing

pub struct HashRing {
    ring: Vec<(u64, ServerThread)>
}

impl HashRing {
    pub fn insert(&mut self, thread: ServerThread) {
        let key = get_key(&thread.virtual_id());
        self.ring.push((key, thread));
        self.ring.sort_by(|(k1, _), (k2, _)| k1.cmp(k2));
    }

    pub fn remove(&mut self, thread: ServerThread) -> Option<ServerThread> {
        let key = get_key(&thread.virtual_id());

        match self.ring.binary_search_by(|(k, _)| k.cmp(&key)) {
            Ok(i) => Some(self.ring.remove(i).1),
            Err(_) => None
        }
    }

    pub fn find(&mut self, input: &String) -> Option<&ServerThread> {
        let key = get_key(&input);

        let n = match self.ring.binary_search_by(|(k, _)| k.cmp(&key)) {
            Err(n) => n,
            Ok(n) => n,
        };

        if n == self.ring.len() {
            return Some(&self.ring[0].1);
        }

        Some(&self.ring[n].1)
    }
}

fn get_key(input: &String) -> u64 {
    let mut hasher = DefaultHasher::new();
    input.hash(&mut hasher);
    return hasher.finish();
}
