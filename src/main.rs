use tonic::{transport::Server, Request, Response, Status};

pub mod greeter {
    tonic::include_proto!("greeter"); // package名に対応
}

use greeter::{
    greeter_server::{Greeter, GreeterServer},
    GoodbyeReply, GoodbyeRequest, HelloReply, HelloRequest,
};

#[derive(Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        let reply = greeter::HelloReply {
            message: format!("Hello, {}!", request.into_inner().name),
        };
        Ok(Response::new(reply))
    }

    async fn say_goodbye(
        &self,
        request: Request<GoodbyeRequest>,
    ) -> Result<Response<GoodbyeReply>, Status> {
        let reply = greeter::GoodbyeReply {
            message: format!("Goodbye, {}!", request.into_inner().name),
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = MyGreeter::default();

    println!("GreeterServer listening on {}", addr);

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
