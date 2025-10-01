use actix_web::{web::{self, get}, App, HttpResponse, HttpServer, Responder};


#[actix_web::main]
async fn main() {
  HttpServer::new(move || {
    App::new()
    .route("/", get().to(|| async {HttpResponse::Ok().body("home")}))
    .service(
      web::scope("/api")
      .route("/hello", get().to(hello))
      .route("/world", get().to(world))
    )
  })
  .bind("0.0.0.0:3000")
  .unwrap()
  .run()
  .await
  .unwrap()
}

async fn hello() -> impl Responder {
  HttpResponse::Ok().body("hello")
}

async fn world() -> impl Responder {
  HttpResponse::Ok().body("world")
}