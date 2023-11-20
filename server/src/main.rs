use actix_web::{App, HttpServer, web, Responder};
use std::net::TcpListener;

async fn home() -> impl Responder {
    "hello from home"
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("localhost:8000")?;
    HttpServer::new(|| 
        App::new()
            .route("/", web::get().to(home))
    )
    .listen(listener)?
    .run()
    .await?;

    Ok(())
}
