use super::model::Todo;
use actix_web::{get, web, HttpResponse, Responder};

#[get("/todos")]
async fn find_all() -> impl Responder {
    HttpResponse::Ok().json(vec![
        Todo {
            id: 1,
            title: String::from("title1"),
            description: String::from("description1"),
            done: false,
        },
        Todo {
            id: 2,
            title: String::from("title2"),
            description: String::from("description2"),
            done: true,
        }
    ])
}

#[get("/todos/{id}")]
async fn find() -> impl Responder {
    HttpResponse::Ok().json(Todo {
        id: 2,
        title: "title".to_string(),
        description: "description".to_string(),
        done: true,
    })
}

// 作成したエンドポイントをserviceにセットして公開
pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(find_all);
    config.service(find);
}
