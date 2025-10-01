use actix_web::{web::{self, ServiceConfig}, App, HttpResponse, HttpServer};

#[allow(unused)]

#[actix_web::main] 
async fn main() {
  HttpServer::new(|| {
    App::new()
    .route("/", web::get().to(||async {HttpResponse::Ok().body("Home")}))
    .service(
      web::scope("/api1")
      .configure(config)
    )
    //reuse config
    .service(
      web::scope("/api2")
      .configure(config)
    )
  })
  .bind("0.0.0.0:3000")
  .unwrap()
  .run()
  .await
  .unwrap()
}

fn config(cfg: &mut ServiceConfig) {
  cfg.service(
    web::scope("/hello")
    .route("/world", web::get().to(|| async {
      HttpResponse::Ok().body("Hello!")
    }))
  );
}