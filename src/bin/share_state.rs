use std::sync::Mutex;

use actix_web::{get, web::{self, get}, App, HttpResponse, HttpServer, Responder};

#[allow(unused)]

#[actix_web::main]
async fn main() {
  let person = Person {name: "Morris".to_string(), age: 20};
  //mutable shared state student
  let student_data = web::Data::new(Student {name: Mutex::new(String::from("Rewel")), age:Mutex::new(20)});
  HttpServer::new(move|| {
    App::new()
    .app_data(web::Data::new(person.clone()))
    .app_data(student_data.clone())
    .route("/", get().to(|| async {HttpResponse::Ok().body("get")}))
    .service(hello)
    .service(student)
    .default_service(web::to(not_found))
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

#[get("/student")]
async fn student(student: web::Data<Student>) -> impl Responder {
  // how to mutate but use this in another route (handler)
  // *student.name.lock().unwrap() = String::from("Rewel");
  // *student.age.lock().unwrap() = 18;
  let name = student.name.lock().unwrap();
  let age = student.age.lock().unwrap();
  let msg = format!("student name is {}, age is {}", name, age);
  HttpResponse::Ok().body(msg)
  //the HttpResponse can take in more status codes ::InternalServerError for 500
}

#[derive(Clone)]
struct Person {
  name: String,
  age: i32
}

//mutable shared state
struct Student {
  name: Mutex<String>,
  age: Mutex<i32>
}


async fn not_found() -> impl Responder {
  HttpResponse::NotFound().body("Not Found")
}