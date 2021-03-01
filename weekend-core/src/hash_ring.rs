use std::collections::{HashMap, HashSet};
use crate::server::ServerThread;
use std::collections::hash_map::{DefaultHasher, Entry};
use std::hash::{Hash, Hasher};
use std::borrow::Borrow;

// Impl of consistent hashing

pub enum HashRingKind {
    GlobalRing,
    LocalRing,
}

pub struct HashRing {
    ring: Vec<(u64, ServerThread)>,
    kind: HashRingKind,
    pub unique_servers: HashMap<String, usize>,
}


impl HashRing {
    #[inline]
    pub fn new_global() -> HashRing {
        return HashRing {
            ring: vec![],
            kind: HashRingKind::GlobalRing,
            unique_servers: HashMap::new()
        };
    }

    #[inline]
    pub fn new_local() -> HashRing {
        return HashRing {
            ring: vec![],
            kind: HashRingKind::LocalRing,
            unique_servers: HashMap::new()
        };
    }
}

impl HashRing {
    pub fn insert(&mut self, thread: ServerThread, join_count: usize) -> bool {
        let entry = self.unique_servers.entry(thread.get_id());

        return match entry {
            Entry::Occupied(mut o) => {
                // If join count is greater then that means this node is trying to rejoin.
                if *o.get() < join_count {
                    o.insert(join_count);
                    true
                } else {
                    false
                }
            }
            Entry::Vacant(v) => {
                v.insert(join_count);
                self.insert_(thread);

                // Todo - add more virtual threads to the ring for better keys distribution
                true
            }
        }
    }

    pub fn remove(&mut self, thread: ServerThread) {
        self.remove_(thread);
        // self.unique_servers.remove(&thread.get_id());
    }

    fn insert_(&mut self, thread: ServerThread) {
        let key = get_key(&self.get_input_key(&thread));
        self.ring.push((key, thread));
        self.ring.sort_by(|(k1, _), (k2, _)| k1.cmp(k2));
    }

    fn remove_(&mut self, thread: ServerThread) -> Option<ServerThread> {
        let key = get_key(&self.get_input_key(&thread));

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

    fn get_input_key(&self, thread: &ServerThread) -> String {
        match self.kind {
            HashRingKind::GlobalRing => thread.get_id(),
            HashRingKind::LocalRing => thread.virtual_id()
        }
    }

    pub fn get_servers(&self) -> impl Iterator<Item=&ServerThread> + '_ {
        return self.ring.iter().map(| (_, val) | val)
    }
}

fn get_key(input: &String) -> u64 {
    let mut hasher = DefaultHasher::new();
    input.hash(&mut hasher);
    return hasher.finish();
}
