use actix_web::{body::MessageBody, dev::{ServiceRequest, ServiceResponse}, get, middleware::{from_fn, Next}, web, App, Error, HttpMessage, HttpRequest, HttpResponse, HttpServer, Responder};

#[allow(unused)]

#[actix_web::main]
async fn main() {
  HttpServer::new(|| {
    App::new()
    .service(hello)
    .service(
      web::scope("/world")
      .route("", web::get().to(world))
      .wrap(from_fn(my_middleware))
    )
    //for a global middle ware put it inside the routes
    // .wrap(from_fn(my_middleware))
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
}

//if you specify the route in the services you do not repeat it here
async fn world(req: HttpRequest) -> impl Responder {
  match req.extensions().get::<Person>() {
    // Some(msg) => HttpResponse::Ok().body(msg.to_string()),
    Some(person) => {
      let msg = format!("Person name: {}, age: {}", person.name, person.age);
      HttpResponse::Ok().body(msg)
    }
    None => HttpResponse::Ok().body("No data")
  }
  // HttpResponse::Ok().body("world")
}


async fn my_middleware(req: ServiceRequest, next: Next<impl MessageBody>) -> Result<ServiceResponse<impl MessageBody>, Error> {
  println!("Hello from middleware");
  //you can remove this line to block the request like this
  // Ok(req.into_response(HttpResponse::Unauthorized().body("UnAuthorized")))

  //passing data from the middleware to handler
  // req.extensions_mut().insert("This is the middleware".to_string());


  //passing custom data to the middleware
  let person = Person{name: String::from("Morris"), age: 23};
  req.extensions_mut().insert(person);


  next.call(req).await
}

//custom data to middleware
struct Person {
  name: String,
  age: i32
}