use std::format;
use dotenv::dotenv;
use env_logger::from_env;
use listenfd::ListenFd;
use std::env;

use actix_web::{get, App, HttpServer, Responder, middleware};

#[get("/hello")]
async fn hello_world() -> impl Responder {
    format!("Hello World")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| {
        App::new()
            .service(hello_world)
            .wrap(middleware::Logger::default())
    });

    env_logger::init();

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host = env::var("HOST").expect("🦀❌ .envにIPアドレスの情報がありません");
            let port = env::var("PORT").expect("🦀❌ .envにポート番号の情報がありません");
            server.bind(format!("{}:{}", host, port))?
        }
    };

    server.run().await
}
