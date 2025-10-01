#[allow(unused)]
use actix_web::{get, http::StatusCode, test, App, HttpResponse, Responder};

#[actix_web::main]
async fn main() {

}

#[get("/hello")]
async fn hello() -> impl Responder {
  HttpResponse::Ok().body("Hello")
}

#[actix_web::test]
async fn test_hello() {
  let app = test::init_service(
    App::new().service(hello)
  ).await;

  let req = test::TestRequest::get().uri("/hello").to_request();

  let res = test::call_service(&app, req).await;

  assert_eq!(res.status(), StatusCode::OK)
}