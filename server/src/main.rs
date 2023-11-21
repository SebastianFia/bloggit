use bloggit::build_server;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("localhost:8000")?;
    build_server(listener)?.await?;
    Ok(())
}
