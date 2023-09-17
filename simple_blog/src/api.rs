use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Post {
    pub id: u64,
    pub title: String,
    pub body: String,
    pub author: String,
    pub published: bool,
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_posts)
        .service(get_post)
        .service(create_post)
        .service(update_post)
        .service(delete_post);
}

#[get("/posts")]
async fn get_posts() -> impl Responder {
    // TODO: Replace this with a call to the database
    HttpResponse::Ok().json(vec![
        Post {
            id: 1,
            title: "Hello".to_string(),
            body: "Hello world!".to_string(),
            author: "John".to_string(),
            published: true,
        },
    ])
}

#[get("/posts/{id}")]
async fn get_post(id: web::Path<u64>) -> impl Responder {
    HttpResponse::Ok().body(format!("Get post {}", id))
}

#[post("/posts")]
async fn create_post(post: web::Json<Post>) -> impl Responder {
    HttpResponse::Ok().json(post.0)
}

#[put("/posts/{id}")]
async fn update_post(id: web::Path<u64>, post: web::Json<Post>) -> impl Responder {
    HttpResponse::Ok().json(Post {
        id: id.into_inner(),
        ..post.into_inner()
    })
}

#[delete("/posts/{id}")]
async fn delete_post(id: web::Path<u64>) -> impl Responder {
    HttpResponse::Ok().body(format!("Delete post {}", id))
}