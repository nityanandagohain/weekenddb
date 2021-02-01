use crate::zmq::ZMQWrapper;
use std::thread;
use std::time::Duration;
use weekend_core::protos::request::KeyRequest;
use protobuf::Message;
use crate::kv_store::KvStore;
use crate::lattices::base_lattices::MapLattice;
use std::collections::HashMap;
use crate::lattices::lww_lattice::LWWLattice;

pub fn run() {
    let context = zmq::Context::new();
    let socket = context.socket(zmq::PULL).unwrap();
    assert!(socket.bind("tcp://*:5555").is_ok());

    let zmq_wrapper = ZMQWrapper{
        socket
    };
    println!("Server started...");

    let map: HashMap<String, LWWLattice<String>> = HashMap::new();

    let kv_store = KvStore{
        db: MapLattice {
            element: map,
            __phantom: Default::default()
        }
    };

    // event loop
    loop {
        let data = zmq_wrapper.recv_bytes();
        let result = KeyRequest::parse_from_bytes(&data).unwrap();

        println!("Received {}", result.request_id);

        thread::sleep(Duration::from_millis(1000));

        println!("No data...");
    }
}