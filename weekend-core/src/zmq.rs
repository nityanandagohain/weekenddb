use zmq::{Socket, PollEvents, Result, Context, SocketType};
use std::error::Error;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::borrow::Borrow;

pub struct ZMQWrapper {
    pub(crate) socket: Socket
}

impl ZMQWrapper {
    pub fn bind(&self, address: &str) -> Result<()> {
        return self.socket.bind(address);
    }

    pub fn connect(&self, address: &str) -> Result<()> {
        return self.socket.connect(address);
    }

    pub fn send_bytes(&self, a: Vec<u8>) {
        let result = self.socket.send(a, 0);
        match result {
            Ok(_) => {}
            Err(e) => { println!("{}", e.to_string()) }
        }
    }

    pub fn send_string(&self, a: &str) {
        let result = self.socket.send(a, 0);
        match result {
            Ok(_) => {}
            Err(e) => { println!("{}", e.to_string()) }
        }
    }

    pub fn recv_bytes(&self) -> Vec<u8> {
        let result = self.socket.recv_bytes(0);
        match result {
            Ok(_) => {}
            Err(e) => { println!("{}", e.to_string()) }
        }
        return result.unwrap();
    }

    pub fn pool(&self, events: PollEvents, timeout_ms: i64) -> Result<i32> {
        return self.socket.poll(events, timeout_ms);
    }
}

pub struct ZMQSocketCache {
    pub cache: HashMap<String, ZMQWrapper>
}

impl ZMQSocketCache {
    pub fn new() -> ZMQSocketCache {
        return ZMQSocketCache{
            cache: HashMap::new()
        }
    }

    pub fn get_or_connect(&mut self, endpoint: String, socket_type: SocketType) -> &ZMQWrapper {
        let socket = self.cache.entry(endpoint.clone());
        return match socket {
            Entry::Occupied(o) => {
                o.into_mut()
            }
            Entry::Vacant(v) => {
                let context = zmq::Context::new();
                let res_socket = context.socket(socket_type).unwrap();
                assert!(res_socket.connect(endpoint.as_str()).is_ok());
                v.insert(ZMQWrapper{
                    socket: res_socket
                })
            }
        }
    }
}