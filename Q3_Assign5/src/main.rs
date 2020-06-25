use actix_web::{HttpResponse,Responder,HttpServer,App,web,Result};
use actix_web::{get,post};
use serde::Deserialize;

#[get("/number/{id}")]
async fn get_number(l: web::Path<(u32,)>) -> impl Responder {
    HttpResponse::Ok().body(format!("Number is {}",l.0))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .service(get_number)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}



