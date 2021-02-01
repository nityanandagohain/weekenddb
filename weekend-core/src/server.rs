use crate::zmq::ZMQWrapper;
use std::thread;
use std::time::Duration;

pub fn run() {
    let context = zmq::Context::new();
    let socket = context.socket(zmq::PULL).unwrap();
    assert!(socket.bind("tcp://*:5555").is_ok());

    let zmq_wrapper = ZMQWrapper{
        socket
    };
    println!("Server started...");


    // event loop
    loop {
        let data = zmq_wrapper.recv_string();

        println!("Received {}", data);
        thread::sleep(Duration::from_millis(1000));

        println!("No data...");
    }
}