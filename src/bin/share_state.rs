use actix_web::{get, web::{self, get}, App, HttpResponse, HttpServer, Responder};

#[allow(unused)]

#[actix_web::main]
async fn main() {
  let person = Person {name: "Morris".to_string(), age: 20};
  HttpServer::new(move|| {
    App::new()
    .app_data(web::Data::new(person.clone()))
    .route("/", get().to(|| async {HttpResponse::Ok().body("get")}))
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
async fn hello(person: web::Data<Person>) -> impl Responder {
  let msg = format!("Person name is {}, age is {}", person.name, person.age);
  HttpResponse::Ok().body(msg)
}

#[get("/world")]
async fn world(person: web::Data<Person>) -> impl Responder {
  let msg = format!("Person name is {}, age is {}", person.name, person.age);
  HttpResponse::Ok().body(msg)
}

#[derive(Clone)]
struct Person {
  name: String,
  age: i32
}