use std::net::TcpListener;
use bloggit::build_server;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("localhost:8000")?;
    build_server(listener)?.await?;
    Ok(())
}
