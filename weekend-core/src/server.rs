use crate::zmq::{ZMQWrapper, ZMQSocketCache};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use weekend_core::protos::request::{KeyRequest, LatticeType, RequestType, SeedResponse, Server};
use protobuf::Message;
use crate::kv_store::KvStore;
use crate::lattices::base_lattices::{MapLattice, Lattice};
use std::collections::HashMap;
use crate::lattices::lww_lattice::{LWWLattice, TimestampValuePair};
use crate::hash_ring::HashRing;


pub struct ServerThread {
    pub public_ip: String,
    pub private_ip: String,
    pub thread_id: usize,
    pub virtual_num: usize,
}

impl ServerThread {
    pub fn virtual_id(&self) -> String {
        return format!("{}:{}/{}", self.private_ip, self.thread_id, self.virtual_num);
    }

    pub fn get_id(&self) -> String {
        return format!("{}:{}", self.private_ip, self.thread_id);
    }

    pub fn get_node_join_addr(&self) -> String {
        return format!("")
    }
}

impl ServerThread {
    pub fn run(&self, seed_ip: String) {
        let mut global_hash_ring: HashRing = HashRing::new_global();
        let mut local_hash_ring: HashRing = HashRing::new_local();
        let data_map: HashMap<String, LWWLattice<Vec<u8>>> = HashMap::new();
        let mut kv_store = KvStore {
            db: MapLattice {
                element: data_map,
                __phantom: Default::default(),
            }
        };
        let mut socket_cache = ZMQSocketCache::new();

        // Request seed node for all the ip address
        let seed_socket = socket_cache.get_or_connect(get_seed_connect_addr(seed_ip), zmq::REQ);
        seed_socket.send_string("Join");
        let data = seed_socket.recv_bytes();
        let seed_response = SeedResponse::parse_from_bytes(&data).unwrap();

        for s in seed_response.servers {
            global_hash_ring.insert(ServerThread{
                public_ip: s.public_ip,
                private_ip: s.private_ip,
                thread_id: 0,
                virtual_num: 0
            }, 0);
        }

        // Todo - get join count for this new server
        global_hash_ring.insert(ServerThread{
            public_ip: self.public_ip.clone(),
            private_ip: self.private_ip.clone(),
            thread_id: 0,
            virtual_num: 0
        }, 0);


        for i in 0..2 {
            local_hash_ring.insert(ServerThread{
                public_ip: self.public_ip.clone(),
                private_ip: self.private_ip.clone(),
                thread_id: i,
                virtual_num: 0
            }, 0);
        }

        // Thread 0 notifies other nodes
        if self.thread_id == 0 {
            self.notify_other_servers(&global_hash_ring, &mut socket_cache);
        }

        // set pool events

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
                                value: tuple.payload,
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
}

// Seed handler

impl ServerThread {

    pub fn notify_other_servers(&self, global_hash_ring: &HashRing, socket_cache: &mut ZMQSocketCache) {
        let mut server = Server::new();
        server.set_public_ip(self.public_ip.clone());
        server.set_private_ip(self.private_ip.clone());

        for s in global_hash_ring.get_servers() {
            let socket = socket_cache.get_or_connect(self.get_node_join_addr(), zmq::PUSH);
            socket.send_bytes(server.write_to_bytes().unwrap());
        }
    }


    pub fn seed_handler(global_hash_ring: &HashRing) -> SeedResponse {
        let mut response = SeedResponse::new();

        for s in global_hash_ring.get_servers() {
            let mut server = Server::new();
            server.set_private_ip(s.private_ip.clone());
            server.set_public_ip(s.public_ip.clone());
            response.servers.push(server);
        }

        return response;
    }
}

fn get_seed_connect_addr(seed_ip: String) -> String {
    return String::from("tcp://{}:{}", seed_ip, 8081);
}
