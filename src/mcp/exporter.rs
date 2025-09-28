use crate::mcp::metrics;
use hyper::{
    service::{make_service_fn, service_fn},
    Body, Request, Response, Server,
};
use std::convert::Infallible;
use std::net::SocketAddr;
use tokio::sync::oneshot;

/// Handle HTTP requests for metrics
async fn metrics_handler(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let metrics = metrics::get_metrics_as_string();
    Ok(Response::new(Body::from(metrics)))
}

/// Start the metrics server
pub async fn start_metrics_server(
    addr: SocketAddr,
    shutdown_signal: oneshot::Receiver<()>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Create a service to handle the metrics endpoint
    let make_svc =
        make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(metrics_handler)) });

    // Create the server
    let server = Server::bind(&addr).serve(make_svc);

    // Add a graceful shutdown
    let server = server.with_graceful_shutdown(async {
        shutdown_signal.await.ok();
    });

    println!("Metrics server listening on http://{}/metrics", addr);

    // Start the server
    server.await?;

    Ok(())
}

/// Create a metrics server with the given port
#[allow(clippy::type_complexity)]
pub fn create_metrics_server(
    port: u16,
) -> (
    tokio::task::JoinHandle<Result<(), Box<dyn std::error::Error + Send + Sync>>>,
    oneshot::Sender<()>,
) {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();

    let handle = tokio::spawn(async move { start_metrics_server(addr, shutdown_rx).await });

    (handle, shutdown_tx)
}
