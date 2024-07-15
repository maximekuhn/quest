use axum::{routing::get, Router};

mod handlers;

fn router() -> Router {
    Router::new().route("/", get(handlers::root))
}

pub async fn start_test_server(
) -> Result<tokio::sync::mpsc::Sender<bool>, Box<dyn std::error::Error + Send + Sync>> {
    let router = router();
    let tcp_listener = tokio::net::TcpListener::bind("127.0.0.1:54378").await?;
    let (tx, rx) = tokio::sync::mpsc::channel(1);
    tokio::spawn(async move {
        let _ = axum::serve(tcp_listener, router)
            .with_graceful_shutdown(shutdown(rx))
            .await;
    });
    Ok(tx)
}

pub async fn shutdown(mut rx: tokio::sync::mpsc::Receiver<bool>) {
    let _ = rx.recv().await;
}
