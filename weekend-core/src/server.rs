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
use zmq::{PollEvents, PollItem};
use std::ops::BitAnd;


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
        return format!("tcp://*:5055")
    }
}

impl ServerThread {
    pub fn run(&self, seed_ip: String) {
        let zmq_context = zmq::Context::new();
        let mut global_hash_ring: HashRing = HashRing::new_global();
        let mut local_hash_ring: HashRing = HashRing::new_local();
        let data_map: HashMap<String, LWWLattice<Vec<u8>>> = HashMap::new();
        let mut kv_store = KvStore {
            db: MapLattice {
                element: data_map,
                __phantom: Default::default(),
            }
        };
        let seed_socket = zmq_context.socket(zmq::REP).unwrap();
        // let zmq_seed_wrapper = ZMQWrapper{socket: seed_socket};
        seed_socket.bind("tcp://1.1.1.1:5056");
        let result = seed_socket.recv_bytes(0);


        let mut socket_cache = ZMQSocketCache::new();

        // Request seed node for all the ip address
        let seed_socke = socket_cache.get_or_connect(get_seed_connect_addr(seed_ip), zmq::REQ);
        seed_socke.send_string("test");
        let data = seed_socke.recv_bytes();
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

        // Defining sockets to pull data from



        let join_pull_socket = zmq_context.socket(zmq::PULL).unwrap();
        join_pull_socket.bind("tcp://*:5057");

        let request_pull_socket = zmq_context.socket(zmq::PULL).unwrap();
        request_pull_socket.bind("tcp://*:5058");

        // Added the above sockets to poll items which then will be used in the event loop
        let mut poll_items: Vec<PollItem> = vec![];
        poll_items.push(seed_socket.as_poll_item(PollEvents::POLLIN));
        poll_items.push(join_pull_socket.as_poll_item(PollEvents::POLLIN));
        poll_items.push(request_pull_socket.as_poll_item(PollEvents::POLLIN));


        // event loop
        loop {
            let poll_result = zmq::poll(&mut poll_items, 0);

            if poll_items[0].get_revents() == PollEvents::POLLIN {
                // handle seed request
                println!("seed request");
                // let vec = zmq_seed_wrapper.recv_bytes();
                // let response = self.seed_handler(&global_hash_ring);
                // zmq_seed_wrapper.send_bytes(response.write_to_bytes().unwrap());
            }

            if poll_items[1].get_revents() == PollEvents::POLLIN {
                // handle node join request
                println!("join request")
            }

            if poll_items[2].get_revents() == PollEvents::POLLIN {
                // handle user request
                println!("get/put request")
            }
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


    pub fn seed_handler(&self, global_hash_ring: &HashRing) -> SeedResponse {
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
    return String::from(format!("tcp://{}:{}", seed_ip, 8081));
}
