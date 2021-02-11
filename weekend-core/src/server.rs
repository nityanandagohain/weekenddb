use crate::zmq::{ZMQWrapper, ZMQSocketCache};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use weekend_core::protos::request::{KeyRequest, LatticeType, RequestType};
use protobuf::Message;
use crate::kv_store::KvStore;
use crate::lattices::base_lattices::{MapLattice, Lattice};
use std::collections::HashMap;
use crate::lattices::lww_lattice::{LWWLattice, TimestampValuePair};

pub fn run() {
    let context = zmq::Context::new();
    let socket = context.socket(zmq::PULL).unwrap();
    assert!(socket.bind("tcp://*:5555").is_ok());

    let mut socket_cache = ZMQSocketCache::new();

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

        let responder_socket = socket_cache.get_or_connect(result.response_address);

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
                    responder_socket.send_string(tuple.key.as_str());
                }
            }
        } else {
            for tuple in result.tuples {
                let k = String::from(tuple.get_key());
                let val = kv_store.get(&k).unwrap().reveal();
            }
        }

        thread::sleep(Duration::from_millis(100));
    }
}
