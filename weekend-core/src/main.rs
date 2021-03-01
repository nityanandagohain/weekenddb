use crate::lattice::Lattice;
use std::collections::HashMap;

pub mod lattices;
mod server;
mod zmq;
mod kv_store;
mod hash_ring;


use crate::lattices::base_lattices as lattice;

fn main() {
    let thread = server::ServerThread {
        public_ip: "1.1.1.1".to_string(),
        private_ip: "1.1.1.1".to_string(),
        thread_id: 0,
        virtual_num: 0
    };

    thread.run("1.1.1.1".to_string())
}
