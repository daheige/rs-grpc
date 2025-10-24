use crate::infras::logger::Logger;
use app::APP_CONFIG;
use autometrics::autometrics;
use infras::metrics::{API_SLO, prometheus_init};
use log::info;
use rust_grpc::hello::greeter_service_server::{GreeterService, GreeterServiceServer};
use rust_grpc::hello::{HelloReply, HelloReq};
use std::net::SocketAddr;
use std::time::Duration;
use tonic::{Request, Response, Status, transport::Server};

mod infras;
/// 定义grpc代码生成的包名
mod rust_grpc;

mod app;

// 这个file descriptor文件是build.rs中定义的descriptor_path路径
// 读取proto file descriptor bin二进制文件
pub(crate) const PROTO_FILE_DESCRIPTOR_SET: &[u8] = include_bytes!("rust_grpc/rpc_descriptor.bin");

/// 实现hello.proto 接口服务
#[derive(Debug, Default)]
pub struct GreeterImpl {}

#[async_trait::async_trait]
impl GreeterService for GreeterImpl {
    // 实现async_hello方法
    #[autometrics(objective = API_SLO)]
    // 也可以使用下面的方式，简单处理
    // #[autometrics]
    async fn say_hello(&self, request: Request<HelloReq>) -> Result<Response<HelloReply>, Status> {
        // 获取request pb message
        let req = &request.into_inner();
        println!("got request.id:{}", req.id);
        println!("got request.name:{}", req.name);
        let reply = HelloReply {
            message: format!("hello,{}", req.name),
            name: format!("{}", req.name).into(),
        };

        Ok(Response::new(reply))
    }
}

/// 采用 tokio 运行时来跑grpc server
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志配置
    Logger::builder().init();
    println!("current pid:{}", std::process::id());

    // 读取配置文件
    info!("app_debug:{}", APP_CONFIG.app_debug);
    let address: SocketAddr = format!("0.0.0.0:{}", APP_CONFIG.grpc_port).parse().unwrap();
    info!("grpc server run on:{}", address);

    // grpc reflection服务，这个可以根据实际情况来开启
    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(PROTO_FILE_DESCRIPTOR_SET)
        .build_v1()
        .unwrap();

    // create http /metrics endpoint
    let metrics_server = prometheus_init(APP_CONFIG.monitor_port);
    let metrics_handler = tokio::spawn(metrics_server);

    // create grpc server
    let greeter = GreeterImpl::default();
    let grpc_handler = tokio::spawn(async move {
        Server::builder()
            .add_service(reflection_service)
            .add_service(GreeterServiceServer::new(greeter))
            .serve_with_shutdown(
                address,
                infras::shutdown::graceful_shutdown(Duration::from_secs(3)),
            )
            .await
            .expect("failed to start grpc server");
    });

    // run async tasks by tokio try_join macro
    let _ = tokio::try_join!(metrics_handler, grpc_handler);
    Ok(())
}
