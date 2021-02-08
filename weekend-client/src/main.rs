use protobuf::Message;
use weekend_core::protos::request::{KeyTuple, KeyRequest, LatticeType, RequestType};

fn main() {
    let context = zmq::Context::new();
    let requester = context.socket(zmq::PUSH).unwrap();

    assert!(requester.connect("tcp://localhost:5555").is_ok());
    let key = String::from("key");
    let val = String::from("value");

    let mut tuple = KeyTuple::new();
    tuple.key = key;
    tuple.lattice_type = LatticeType::LWW;
    tuple.payload = val.into_bytes();

    let mut request_body = KeyRequest::new();
    request_body.request_id = String::from("unique");
    request_body.field_type = RequestType::PUT;
    request_body.tuples.push(tuple);

    for request_nbr in 0..10 {
        println!("Sending Hello {}...", request_nbr);
        requester.send(request_body.write_to_bytes().unwrap(), 0);
    }
}
