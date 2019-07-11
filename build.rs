fn main() {
    let proto_root = "src/protos";
    println!("cargo:rerun-if-changed={}{}", proto_root, "/rpc.proto");

    protoc_rust_grpc::run(protoc_rust_grpc::Args {
        out_dir: "src/protos",
        includes: &["src/protos"],
        input: &["src/protos/rpc.proto"],
        rust_protobuf: true, // also generate protobuf messages, not just services
        ..Default::default()
    }).expect("Failed to compile gRPC definitions!");
}
