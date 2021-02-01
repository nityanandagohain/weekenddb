use zmq::{Socket, PollEvents, Result};
use std::error::Error;

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

    pub fn send_string(&self, a: &str) {
        let result = self.socket.send(a, 0);
        match result {
            Ok(_) => {}
            Err(e) => {println!("{}", e.to_string())}
        }
    }

    pub fn recv_string(&self) -> String {
        let mut msg = zmq::Message::new();
        let result = self.socket.recv(&mut msg, 0);
        match result {
            Ok(_) => {}
            Err(e) => {println!("{}", e.to_string())}
        }

        return String::from(msg.as_str().unwrap());
    }

    pub fn pool(&self, events: PollEvents, timeout_ms: i64) -> Result<i32> {
        return self.socket.poll(events, timeout_ms);
    }
}