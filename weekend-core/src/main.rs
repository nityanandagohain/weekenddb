use crate::lattice::Lattice;
use std::collections::HashMap;

pub mod lattices;
mod server;
mod zmq;
mod kv_store;
mod hash_ring;


use crate::lattices::base_lattices as lattice;

fn main() {
    server::run()
}


// fn main() {
//     println!("Hello World!");
//
//     let mut mx_lat = lattice::MaxLattice{
//         element: 4
//     };
//
//     let mut test_map = HashMap::new();
//     test_map.insert(
//         "k", mx_lat
//     );
//
//     let mut map_lattice = lattice::MapLattice{
//         element: test_map,
//         __phantom: Default::default()
//     };
//
//     let new_elem = lattice::MaxLattice{
//         element: 5
//     };
//
//     let mut test_map_2 = HashMap::new();
//     test_map_2.insert(
//         "k", new_elem
//     );
//
//     map_lattice.merge_elem(&test_map_2);
//
//     for (book, review) in map_lattice.reveal() {
//         println!("{}: \"{}\"", book, review.reveal());
//     }
// }