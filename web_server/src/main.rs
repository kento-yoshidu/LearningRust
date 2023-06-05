use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

struct AppState {
    author_name: String
}

#[get("/")]
async fn hello(data: web::Data<AppState>) -> impl Responder {
    let author_name = &data.author_name;
    HttpResponse::Ok().body(format!("Hello World {author_name}"))
}

#[post("/echo")]
async fn echo(req_boby: String) -> impl Responder {
    HttpResponse::Ok().body(req_boby + "!!!")
}

#[get("/state")]
async fn state(data: web::Data<AppState>) -> String {
    let author_name = &data.author_name;
    format!("Hello {author_name}")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState {
                author_name: String::from("by Kento Yoshizu")
            }))
            .service(hello)
            .service(echo)
            .service(state)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
