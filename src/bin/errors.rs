use std::fmt::Display;

use actix_web::{error::ErrorBadRequest, web::Path, Error, ResponseError};
#[allow(unused)]
use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[actix_web::main]
async fn main() {
  HttpServer::new(|| {
    App::new()
    .service(hello)
    .service(world)
  })
  .bind("0.0.0.0:3000")
  .unwrap()
  .run()
  .await
  .unwrap()
}

#[get("/hello/{id}")]
async fn hello(id: Path<i32>) -> Result<impl Responder, Error> {
  let id = id.into_inner();
  if id < 10 {
    Ok(HttpResponse::Ok().body("hello"))
  }else {
    Err(ErrorBadRequest("Bad Request"))
  }
}

#[get("/world")]
async fn world() -> Result<String, MyError> {
  Err(MyError { message: "something went wrong...".to_string() })
}

#[derive(Debug)]
struct MyError {
  message: String
}

impl Display for MyError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Error message: {}", self.message)
  }
}

impl ResponseError for MyError {}