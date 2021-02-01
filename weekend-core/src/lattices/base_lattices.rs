use std::collections::hash_map::RandomState;
use std::marker::PhantomData;
use std::hash::Hash;
use std::collections::HashMap;

pub trait Lattice<T> {
    fn reveal(&self) -> &T;
    fn merge_elem(&mut self, t: &T);
    fn merge(&mut self, t: &impl Lattice<T>) {
        self.merge_elem(t.reveal())
    }
}


// Max lattice
pub struct MaxLattice<T: PartialOrd> {
    pub element: T
}

// Max lattice impl
impl<T: PartialOrd + Copy> Lattice<T> for MaxLattice<T> {
    fn reveal(&self) -> &T {
        return &self.element;
    }

    fn merge_elem(&mut self, t: &T) {
        if self.element < *t {
            self.element = *t
        }
    }
}

impl<T: PartialOrd + std::ops::Add<Output=T> + std::ops::Sub<Output=T> + Copy> MaxLattice<T> {
    pub fn add(&self, t: &T) -> MaxLattice<T> {
        return MaxLattice {
            element: self.element + *t
        };
    }

    pub fn subtract(&self, t: &T) -> MaxLattice<T> {
        return MaxLattice {
            element: self.element - *t
        };
    }
}

impl<T: PartialOrd + Copy> Clone for MaxLattice<T> {
    fn clone(&self) -> Self {
        return MaxLattice {
            element: self.element
        };
    }

    fn clone_from(&mut self, source: &Self) {
        self.element = source.element;
    }
}

// Bool lattice
pub struct BoolLattice {
    element: bool
}

// bool lattice impl
impl Lattice<bool> for BoolLattice {
    fn reveal(&self) -> &bool {
        return &self.element;
    }

    fn merge_elem(&mut self, t: &bool) {
        self.element |= t
    }
}

pub struct MapLattice<K, V, T> where V: Lattice<T> {
    pub element: HashMap<K, V>,
    pub __phantom: PhantomData<T>, // the name "__phantom" doesn't matter
}

impl<K: Eq + Hash + Clone, V, T> Lattice<HashMap<K, V>> for MapLattice<K, V, T> where V: Lattice<T> + Clone
{
    fn reveal(&self) -> &HashMap<K, V, RandomState> {
        return &self.element;
    }

    fn merge_elem(&mut self, t: &HashMap<K, V, RandomState>) {
        for (key, value) in t.into_iter() {
            self.insert(key, value)
        }
    }
}

impl<K: Eq + Hash + Clone, V, T> MapLattice<K, V, T> where V: Lattice<T> + Clone
{
    pub fn insert(&mut self, k: &K, v: &V) {
        let val = self.element.get_mut(&k);
        if val.is_some() {
            let curr_val = val.unwrap();
            curr_val.merge(v);
        } else {
            self.element.insert(k.clone(), v.clone());
        }
    }
}
