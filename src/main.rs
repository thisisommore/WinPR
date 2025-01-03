use tonic::{transport::Server, Request, Response, Status};

use world::world_server::{World, WorldServer};
use world::{CreateWorldReply, CreateWorldRequest};

pub mod world {
    tonic::include_proto!("world");
}

#[derive(Default)]
pub struct MyWorld {}

#[tonic::async_trait]
impl World for MyWorld {
    async fn create_world(
        &self,
        request: Request<CreateWorldRequest>,
    ) -> Result<Response<CreateWorldReply>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let reply = world::CreateWorldReply {
            welcome_message: format!("Hello god speed {}!", request.into_inner().member),
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let greeter = MyWorld::default();

    println!("GreeterServer listening on {}", addr);

    Server::builder()
        .add_service(WorldServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
