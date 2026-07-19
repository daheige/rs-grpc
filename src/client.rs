use rust_grpc::hello::HelloReq;
use rust_grpc::hello::greeter_client::GreeterClient;

// tonic request
use tonic::Request;

mod rust_grpc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let request = Request::new(HelloReq {
        name: "daheige".into(),
    });

    let address = "http://127.0.0.1:50051";
    // let address = "http://127.0.0.1:8081";
    let mut client = GreeterClient::connect(address).await?;
    println!("client:{:?}", client);

    let response = client.say_hello(request).await?;
    println!("res:{:?}", response);

    let res = response.into_inner();
    println!("message:{}", res.message);
    Ok(())
}
