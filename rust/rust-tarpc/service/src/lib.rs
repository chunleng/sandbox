#[tarpc::service]
pub trait HelloService {
    async fn hello(name: String) -> String;
}
