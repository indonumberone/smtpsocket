use anyhow::{Context, Result};
use tokio::net::TcpListener;

use std::env;

use server::smtp;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "0.0.0.0:2525".to_string());

    let domain = &env::args()
        .nth(2)
        .unwrap_or_else(|| "kelompok1.com".to_string());

    tracing::info!("server server for {domain} started");

    let listener = TcpListener::bind(&addr).await?;
    tracing::info!("Listening on: {}", addr);

    // Task for deleting old mail
    // periodically_clean_db(tokio::time::Duration::from_secs(3600));

    // Main loop: accept connections and spawn a task to handle them
    loop {
        let (stream, addr) = listener.accept().await?;
        tracing::info!("Accepted a connection from {}", addr);

        tokio::task::LocalSet::new()
            .run_until(async move {
                let smtp = smtp::Server::new(domain, stream).await?;
                tokio::time::timeout(std::time::Duration::from_secs(300), smtp.serve())
                    .await
                    .context("connection timed out")
            })
            .await
            .ok();
    }
}
