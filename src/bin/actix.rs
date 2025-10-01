use actix_web::{get, post, web::{self, delete, get, post, put, Json}, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[actix_web::main]
async fn main() {
    HttpServer::new(|| {
        App::new()
        .route("/", get().to(|| async {HttpResponse::Ok().body("hello world".to_string())}))
        .route("/hello", get().to(|| async {HttpResponse::Ok().body("hello")}))
        .route("/", post().to(|| async {HttpResponse::Ok().body("post")}))
        .route("/", delete().to(|| async {HttpResponse::Ok().body("delete")}))
        .route("/", put().to(|| async {HttpResponse::Ok().body("put")}))
        .service(contact)
        .service(user_contact)
        .service(hello)
        .service(add_user)
        .service(hi)
    })
    .bind("0.0.0.0:3000")
    .unwrap()
    .run()
    .await
    .unwrap()
}

#[get("/contact")]
async fn contact() -> impl Responder {
    HttpResponse::Ok().body("contact")
}

//dynamic route
#[get("/contact/{id}")]
async fn user_contact(path: web::Path<i32>) -> impl Responder {
    let id: i32 = path.into_inner();
    let msg: String = format!("id is {}", id);
    HttpResponse::Ok().body(msg)
}

//query parameters /user?name=name&age=age
#[get("/user")]
async fn hello(info: web::Query<Info>) -> impl Responder {
    let msg: String = format!("name: {}, age: {}", info.name, info.age);
    HttpResponse::Ok().body(msg)
}

#[derive(Deserialize)]
struct Info {
    name: String,
    age: i32
}

//request json 
#[post("/add_user")]
async fn add_user(user: Json<Info>) -> impl Responder {
    let msg = format!("name: {}, age: {}", user.name, user.age);
    HttpResponse::Ok().body(msg)
}


//response json
#[get("/hi")]
async fn hi() -> impl Responder {
    let person = Person {name: "Morris".to_string(), age: 20};
    let person_json = serde_json::to_string(&person).unwrap();
    HttpResponse::Ok().json(person_json)

}

#[derive(Serialize)]
struct Person {
    name: String,
    age: i32
}