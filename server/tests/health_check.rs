use bloggit::build_server;
use std::net::TcpListener;

fn spawn_app() -> String {
    let listener = TcpListener::bind("localhost:8000").expect("failed to bind to random port");
    let address = listener.local_addr().expect("Failed to get address from TcpListener").to_string();
    println!("Address: {}", address);
    tokio::spawn(build_server(listener).expect("failed to build server"));

    format!("http:{}", address)
}

#[tokio::test]
async fn health_check() {
    let app_address = spawn_app();
    let client = reqwest::Client::new();
    let health_check_url = format!("{}/health_check", app_address);
    println!("app address: {}", app_address);

    let response = client 
        .get(health_check_url)
        .send()
        .await
        .expect("failed to send request");

    assert_eq!(200, response.status().as_u16());
}
