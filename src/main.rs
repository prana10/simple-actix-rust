use actix_web::{HttpServer, App, get, Responder, HttpResponse, web};


// simple api ping using actix rust
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(||{
        App::new()
            .service(ping)
            .route("/hello", web::get().to(hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}


#[get("/")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().body("pong")
}

async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world")
}