#![allow(unused)]

use actix_web::{get, guard, web, App, HttpRequest, HttpResponse, HttpServer, Responder};

#[actix_web::main]
async fn main() {
  HttpServer::new(|| {
    App::new()
    .service(
      web::scope("/api")
      //guard will only allow get requests
      .guard(guard::Get())
      .route("/hello/{a:.*}", web::get().to(hello))
      //this will return a 404 not found error in postman since post request is guarded
      .route("/world", web::post().to(world))
    )
  })
  .bind("0.0.0.0:3000")
  .unwrap()
  .run()
  .await
  .unwrap()
}

//.* mean it will match wildcards(one or more other characters, values or entities in various contexts)
// #[get("/hello/{a:.*}")]
async fn hello(req: HttpRequest) -> impl Responder {
  let path = req.match_info().query("a");
  let msg = format!("path: {}", path);
  HttpResponse::Ok().body(msg)
}

async fn world() -> impl Responder {
  HttpResponse::Ok().body("world")
}