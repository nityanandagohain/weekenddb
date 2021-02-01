extern crate protoc_rust;

use protoc_rust::Customize;

fn main() {
    protoc_rust::Codegen::new()
        .customize(Customize {
            gen_mod_rs: Some(true),
            ..Default::default()
        })
        .out_dir("src/protos")
        .inputs(&["protos/lattice.proto", "protos/request.proto"])
        .include("protos")
        .run()
        .expect("protoc");
}