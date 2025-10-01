#![allow(unused)]

use actix_web::{get, App, HttpRequest, HttpResponse, HttpServer, Responder};

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

//.* mean it will match wildcards(one or more other characters, values or entities in various contexts)
#[get("/hello/{a:.*}")]
async fn hello(req: HttpRequest) -> impl Responder {
  let path = req.match_info().query("a");
  let msg = format!("path: {}", path);
  HttpResponse::Ok().body(msg)
}