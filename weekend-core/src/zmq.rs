use zmq::{Socket, PollEvents, Error, Result};

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
        self.socket.send(a, 0).unwrap();
    }

    pub fn recv_string(&self) -> String {
        let mut msg = zmq::Message::new();
        self.socket.recv(&mut msg, 0).unwrap();
        return String::from(msg.as_str().unwrap());
    }

    pub fn pool(&self, events: PollEvents, timeout_ms: i64) -> Result<i32> {
        return self.socket.poll(events, timeout_ms);
    }
}