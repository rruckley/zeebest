fn main() {
    println!("cargo:rerun-if-changed=proto/gateway.proto");

    protoc_rust_grpc::Codegen::new()
    .out_dir("src")
    .input("proto/gateway.proto")
    .rust_protobuf(true)
    .run()
        .expect("Failed to generate from ProtoBuf");
}
