use crate::rust_grpc::hello::{HealthzReply, HealthzReq};
use autometrics::autometrics;
use axum::extract::{Path, State};
use axum::{
    Json, Router, extract::Request as AxumRequest, http::StatusCode, http::header::CONTENT_TYPE,
    response::IntoResponse, routing::get,
};
use monitor::metrics::prometheus_init;
use rust_grpc::hello::greeter_server::{Greeter, GreeterServer};
use rust_grpc::hello::{HelloReply, HelloReq};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpListener;
use tonic::service::Routes;
use tonic::{Request, Response, Status};
use tower::{make::Shared, steer::Steer};

mod rust_grpc;

// 这个file descriptor文件是build.rs中定义的descriptor_path路径
// 读取proto file descriptor bin二进制文件
pub(crate) const PROTO_FILE_DESCRIPTOR_SET: &[u8] = include_bytes!("rust_grpc/rpc_descriptor.bin");

/// 实现hello.proto 接口服务
#[derive(Debug, Default, Clone)]
pub struct GreeterImpl {}

#[derive(Clone)]
pub struct AppState {
    greeter: GreeterImpl,
}

fn new_greeter() -> GreeterImpl {
    let greeter = GreeterImpl::default();
    greeter
}

#[async_trait::async_trait]
impl Greeter for GreeterImpl {
    // 实现async_hello方法
    #[autometrics]
    async fn say_hello(&self, request: Request<HelloReq>) -> Result<Response<HelloReply>, Status> {
        // 获取request pb message
        let req = &request.into_inner();
        println!("got request.name:{}", req.name);
        let reply = HelloReply {
            message: format!("hello,{}", req.name),
        };

        Ok(Response::new(reply))
    }

    #[autometrics]
    async fn healthz(
        &self,
        request: Request<HealthzReq>,
    ) -> Result<Response<HealthzReply>, Status> {
        let req = request.into_inner();
        println!("req:{:?}", req);

        let current_time = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let reply = HealthzReply {
            current_time,
            alive: true,
        };

        Ok(Response::new(reply))
    }
}

async fn web_root() -> &'static str {
    "Hello, World!"
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Reply<T> {
    pub code: i32,
    pub message: String,
    pub data: Option<T>,
}

// 将请求反序列化到HelloReq，然后调用grpc service
#[autometrics]
async fn say_hello(
    Path(name): Path<String>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    println!("got name:{}", name);
    let req = Request::new(HelloReq { name });
    let response = state.greeter.say_hello(req).await;
    match response {
        Ok(res) => {
            let reply = res.into_inner();
            (
                StatusCode::OK,
                Json(Reply {
                    code: 0,
                    message: "ok".to_string(),
                    data: Some(reply),
                }),
            )
        }
        Err(err) => (
            StatusCode::OK,
            Json(Reply {
                code: 500,
                message: format!("request err:{}", err),
                data: None,
            }),
        ),
    }
}

/// 采用 tokio 运行时来跑grpc server
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address: SocketAddr = "0.0.0.0:8081".parse()?;
    println!("grpc server and http server run on:{}", address);

    // grpc reflection服务
    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(PROTO_FILE_DESCRIPTOR_SET)
        .build_v1()?;

    // grpc service
    let greeter = new_greeter();
    let greeter_clone = greeter.clone();
    let app_state = Arc::new(AppState {
        greeter: greeter_clone,
    });
    let grpc_service = GreeterServer::new(greeter);

    // create grpc server
    let grpc_server = Routes::new(grpc_service)
        .add_service(reflection_service)
        .into_axum_router();

    // build the rest service
    let rest_server = Router::new()
        .route("/", get(web_root))
        .route("/v1/greeter/say/{name}", get(say_hello))
        .with_state(app_state);

    // combine them into one service
    let service = Steer::new(
        vec![rest_server, grpc_server],
        |req: &AxumRequest, _services: &[_]| {
            if req
                .headers()
                .get(CONTENT_TYPE)
                .map(|content_type| content_type.as_bytes())
                .filter(|content_type| content_type.starts_with(b"application/grpc"))
                .is_some()
            {
                // grpc service
                1
            } else {
                // http service
                0
            }
        },
    );

    // create http /metrics endpoint
    let metrics_server = prometheus_init(8092);
    let metrics_handler = tokio::spawn(metrics_server);
    let multiplex_handler = tokio::spawn(async move {
        // run multiplex service on one port
        let listener = TcpListener::bind(&address).await.unwrap();
        axum::serve(listener, Shared::new(service))
            .with_graceful_shutdown(shutdown::graceful_shutdown(Duration::from_secs(3)))
            .await
            .expect("failed to start multiplex service");
    });

    // run async tasks by tokio try_join macro
    let _ = tokio::try_join!(metrics_handler, multiplex_handler);
    Ok(())
}
