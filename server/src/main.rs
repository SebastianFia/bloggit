use actix_web::{web, App, HttpServer};
use bloggit::routes::home;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("localhost:8000")?;
    HttpServer::new(|| App::new().route("/", web::get().to(home)))
        .listen(listener)?
        .run()
        .await?;

    Ok(())
}
