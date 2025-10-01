#[allow(unused)]

use actix_web::http::header::ContentType;
use actix_web::{body::BoxBody, get, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;

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

#[get("/hello")]
async fn hello() -> impl Responder {
  Person {
    name: String::from("Morris"),
    age: 23
  }

}

#[derive(Serialize)]
struct Person {
  name: String,
  age: i32
}

impl Responder for Person {
  type Body = BoxBody;
  fn respond_to(self, _req: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
      let body = serde_json::to_string_pretty(&self).unwrap();
      HttpResponse::Ok().content_type(ContentType::json()).body(body)
  }
}