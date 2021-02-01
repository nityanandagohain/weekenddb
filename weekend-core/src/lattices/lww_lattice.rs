use crate::lattices::base_lattices::Lattice;

pub struct TimestampValuePair<T> {
    timestamp: u64,
    value: T
}

impl<T> TimestampValuePair<T> {
    #[inline]
    fn new(val: T) -> TimestampValuePair<T> {
        return TimestampValuePair{
            timestamp: 0,
            value: val
        };
    }

    #[inline]
    fn new_with_timestamp(val: T, ts: u64) -> TimestampValuePair<T> {
        return TimestampValuePair{
            timestamp: ts,
            value: val
        };
    }

    // TODO - Figure out why size is required
}

pub struct LWWLattice<T> {
    element: TimestampValuePair<T>
}

impl<T: Copy> Lattice<TimestampValuePair<T>> for LWWLattice<T> {
    fn reveal(&self) -> &TimestampValuePair<T> {
        return &self.element;
    }

    fn merge_elem(&mut self, t: &TimestampValuePair<T>) {
        if t.timestamp >= self.element.timestamp {
            self.element.timestamp = t.timestamp;
            self.element.value = t.value;
        }
    }
}
