use actix_web::{HttpServer, App, web, Responder};
use std::io;

mod models;
use crate::models::Status;

async fn status() -> impl Responder {
   web::HttpResponse::Ok()
                .json(Status {status: "Ok".to_string()})
}

#[actix_rt::main]
async fn main() -> io::Result<()>{

    println! ("Server running on 127.0.0.1:8000");

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(status))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await

   
}
