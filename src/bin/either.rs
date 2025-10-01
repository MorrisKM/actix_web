use actix_web::{web::Path, Either};
#[allow(unused)]
use actix_web::{get, App, HttpResponse, HttpServer, Responder};

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

#[get("/hello/{id}")]
async fn hello(id: Path<i32>) -> Either<String, HttpResponse> {
  let id = id.into_inner();
  if id < 10 {
    Either::Left(String::from("hello"))
  }else {
    Either::Right(HttpResponse::Ok().body("world"))
  }
}

