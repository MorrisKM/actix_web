use actix_web::{error::ErrorBadRequest, web::Path, Error};
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
async fn hello(id: Path<i32>) -> Result<impl Responder, Error> {
  let id = id.into_inner();
  if id < 10 {
    Ok(HttpResponse::Ok().body("hello"))
  }else {
    Err(ErrorBadRequest("Bad Request"))
  }
}