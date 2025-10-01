use actix_web::{get, web::{self, Redirect}, App, HttpResponse, HttpServer, Responder};

#[allow(unused)]

#[actix_web::main]
async fn main() {
  HttpServer::new(|| {
    App::new()
    //first way to add a redirect
    .service(web::redirect("/hello", "/world"))
    .service(hello)
    .service(world)
  })
  .bind("0.0.0.0:3000")
  .unwrap()
  .run()
  .await
  .unwrap()
}

#[get("/hello")]
async fn hello() -> impl Responder {
  HttpResponse::Ok().body("hello")

  //second way to add a redirect
  // web::redirect("/hello", "/world")

  //third way to redirect using the Redirect import
  // Redirect::to("/world")
}

#[get("/world")]
async fn world() -> impl Responder {
  HttpResponse::Ok().body("world")
}