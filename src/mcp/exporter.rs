use crate::mcp::metrics;
use bytes::Bytes;
use hyper::{Request, Response};
use hyper::body::Incoming;
use http_body_util::Full;
use hyper::service::service_fn;
use hyper::server::conn::http1;
use hyper_util::rt::TokioIo;
use std::convert::Infallible;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio::sync::oneshot;
use tokio::time::{sleep, Duration};

/// Handle HTTP requests for metrics
async fn metrics_handler(_req: Request<Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    let metrics = metrics::get_metrics_as_string();
    Ok(Response::new(Full::new(Bytes::from(metrics))))
}

/// Start the metrics server
pub async fn start_metrics_server(
    addr: SocketAddr,
    mut shutdown_signal: oneshot::Receiver<()>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Bind TCP listener
    let listener = TcpListener::bind(addr).await?;

    println!("Metrics server listening on http://{}/metrics", addr);

    loop {
        tokio::select! {
            _ = &mut shutdown_signal => {
                // Graceful shutdown: stop accepting new connections
                println!("Shutting down metrics server");
                break;
            }
            accept_res = listener.accept() => {
                let (stream, _peer_addr) = accept_res?;
                let io = TokioIo::new(stream);

                // Create a service to handle the metrics endpoint per-connection
                let service = service_fn(metrics_handler);

                // Spawn a task to serve this connection using HTTP/1
                tokio::spawn(async move {
                    if let Err(err) = http1::Builder::new()
                        .keep_alive(true)
                        .serve_connection(io, service)
                        .await
                    {
                        eprintln!("Error serving metrics connection: {err}");
                    }
                });
            }
        }
    }

    // Give some time for spawned tasks to finish sending responses (best-effort)
    sleep(Duration::from_millis(50)).await;

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
