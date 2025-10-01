use actix_web::{post, web::Form, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[allow(unused)]


#[actix_web::main]
async fn main() {
  HttpServer::new(|| {
    App::new()
    .service(hello)
  })
  .bind("0.0.0.0:3000")
  .unwrap()
  .run()
  .await
  .unwrap()
}

#[post("/hello")]
async fn hello(info: Form<Info>) -> impl Responder {
  let msg = format!("name: {}, age: {}", info.name, info.age);
  HttpResponse::Ok().body(msg)
}

#[derive(Deserialize)]
struct Info {
  name: String,
  age: i32
}