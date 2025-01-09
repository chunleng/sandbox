fn main() {
    tonic_build::compile_protos("proto/coffee_service.proto").unwrap();
}
