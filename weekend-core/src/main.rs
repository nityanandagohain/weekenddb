use crate::lattice::Lattice;
use std::collections::HashMap;

pub mod lattices;
mod server;
mod zmq;
mod kv_store;
mod hash_ring;


use crate::lattices::base_lattices as lattice;

fn main() {
    let mut settings = config::Config::default();
    settings
        .merge(config::Environment::with_prefix("weekend")).unwrap();

    let thread = server::ServerThread {
        public_ip: "1.1.1.1".to_string(),
        private_ip: settings.get("private_ip").unwrap(),
        thread_id: 0,
        virtual_num: 0,
    };

    thread.run(settings.get("seed_ip").unwrap(), settings.get("is_seed_node").unwrap())
}
