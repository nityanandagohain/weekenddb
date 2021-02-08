use crate::zmq::ZMQWrapper;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use weekend_core::protos::request::{KeyRequest, LatticeType, RequestType};
use protobuf::Message;
use crate::kv_store::KvStore;
use crate::lattices::base_lattices::MapLattice;
use std::collections::HashMap;
use crate::lattices::lww_lattice::{LWWLattice, TimestampValuePair};

pub fn run() {
    let context = zmq::Context::new();
    let socket = context.socket(zmq::PULL).unwrap();
    assert!(socket.bind("tcp://*:5555").is_ok());

    let zmq_wrapper = ZMQWrapper{
        socket
    };
    println!("Server started...");

    let map: HashMap<String, LWWLattice<Vec<u8>>> = HashMap::new();

    let mut kv_store = KvStore{
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

        if result.field_type == RequestType::PUT {
            for tuple in result.tuples {
                if tuple.lattice_type == LatticeType::LWW {
                    let l = LWWLattice {
                        element: TimestampValuePair {
                            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis(),
                            value: tuple.payload
                        }
                    };

                    kv_store.put(&tuple.key, &l);
                }
            }
        } else {

        }

        thread::sleep(Duration::from_millis(100));
    }
}