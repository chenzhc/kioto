use hello_world::greeter_client::GreeterClient;
use hello_world::HelloRequest;
use log::info;
use tokio_stream::StreamExt;

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    kioto::init();
    let mut client = GreeterClient::connect("http://localhost:50051").await?;

    // Unary RPC: SayHello
    let request = tonic::Request::new(HelloRequest {
        name: "Alice".into(),
    });

    let response = client.say_hello(request).await?;
    info!("Unary Response: {}", response.into_inner().message);


    // Streaming RPC: SayHelloStream
    let request = tonic::Request::new(HelloRequest {
        name: "Bob".to_string(),
    });
    let mut stream = client.say_hello_stream(request).await?.into_inner();

    while let Some(response) = stream.next().await {
        match response {
            Ok(reply) => {
                info!("Stream Response: {}", reply.message);
            }
            Err(e) => info!("Stream Error: {}", e),
        }
    }

    Ok(())
}
