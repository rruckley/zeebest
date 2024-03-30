
fn main() {
    println!("cargo:rerun-if-changed=proto/gateway.proto");

    // protoc_rust_grpc::Codegen::new()
    // .out_dir("src")
    // .input("proto/gateway.proto")
    // .rust_protobuf(true)
    // .run()
    //     .expect("Failed to generate from ProtoBuf");

    protobuf_codegen::Codegen::new()
    // Use `protoc` parser, optional.
    .protoc()
    // Use `protoc-bin-vendored` bundled protoc command, optional.
    //.protoc_path(&protoc_bin_vendored::protoc_bin_path().unwrap())
    // All inputs and imports from the inputs must reside in `includes` directories.
    .includes(&["proto"])
    // Inputs must reside in some of include paths.
    .input("proto/gateway.proto")
    // Specify output directory relative to Cargo output directory.
    .cargo_out_dir("src")
    .run_from_script();
}
