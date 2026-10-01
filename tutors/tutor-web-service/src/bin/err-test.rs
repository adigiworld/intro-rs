use actix_web::{App, HttpResponse, HttpServer, error::Error, web};
use std::fs::File;

async fn hello() -> Result<HttpResponse, Error> {
    let _ = File::open("fictionalfile.txt")?;
    Ok(HttpResponse::Ok().body("Hello there!"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/hello", web::get().to(hello)))
        .bind("127.0.0.1:3000")?
        .run()
        .await
}
