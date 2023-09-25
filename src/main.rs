use std::net::TcpListener;

use zero2prod::run;

#[tokio::main] //main has no poll method, so it can't be used as future, thus we need to use tokio::main
async fn main() -> std::io::Result<()> {
    // The io:Error gets propagated up here now
    // If no Error, call .await on our Server
    let listener = TcpListener::bind("127.0.0.1:8000").expect("Failed to bind random port");
    run(listener)?.await
}
