use tonic::{transport::Server, Request, Response, Status};

pub mod hello_world {
    tonic::include_proto!("passwdb"); // The string specified here must match the proto package name
}

use hello_world::{
    server::{Vault, VaultServer},
    HelloReply, HelloRequest,
};

pub struct MyVault {}

#[tonic::async_trait]
impl Vault for MyVault {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<HelloReply>, Status> { // Return an instance of type HelloReply
        println!("Got a request: {:?}", request);

        let reply = hello_world::HelloReply {
            message: format!("Hello {}!", request.into_inner().name).into(), // We must use .into_inner() as the fields of gRPC requests and responses are private
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let vault = MyVault {};

    Server::builder()
        .add_service(VaultServer::new(vault))
        .serve(addr)
        .await?;

    Ok(())
}
