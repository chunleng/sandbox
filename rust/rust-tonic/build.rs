fn main() {
    tonic_build::compile_protos("proto/coffee_service.proto").unwrap();
    tonic_build::compile_protos("proto/chat_service.proto").unwrap();
}
