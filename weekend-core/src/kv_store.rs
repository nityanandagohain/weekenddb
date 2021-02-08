use crate::lattices::base_lattices::{MapLattice, Lattice};
use std::hash::Hash;

pub struct KvStore<K, V, T> where V: Lattice<T> {
    pub db: MapLattice<K, V, T>
}

impl<K: Eq + Hash + Clone, V, T> KvStore<K, V, T> where V: Lattice<T> + Clone {
    pub fn get(&self, key: &K) -> Option<&V> {
        self.db.element.get(key)
    }

    pub fn put(&mut self, key: &K, val: &V) {
        self.db.insert(key, val);
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.db.element.remove(key)
    }
}
