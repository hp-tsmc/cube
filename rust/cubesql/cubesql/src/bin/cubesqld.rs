use cubesql::{
    config::{processing_loop::ShutdownMode, Config, CubeServices},
    telemetry::{LocalReporter, ReportingLogger},
};

use log::Level;
use simple_logger::SimpleLogger;
use std::{env, sync::Arc};
use std::net::SocketAddr;

use tokio::runtime::Builder;
use prometheus::{Registry, TextEncoder, Encoder};
use hyper::{Server, service::{make_service_fn, service_fn}, Body, Request, Response};

fn main() {
    println!("Starting cubesql");
    let log_level = match env::var("CUBESQL_LOG_LEVEL")
        .unwrap_or("info".to_string())
        .to_lowercase()
        .as_str()
    {
        "error" => Level::Error,
        "warn" => Level::Warn,
        "info" => Level::Info,
        "debug" => Level::Debug,
        "trace" => Level::Trace,
        x => panic!("Unrecognized log level: {}", x),
    };
    let logger = SimpleLogger::new()
        .with_level(Level::Error.to_level_filter())
        .with_module_level("cubeclient", log_level.to_level_filter())
        .with_module_level("cubesql", log_level.to_level_filter())
        .with_module_level("datafusion", Level::Warn.to_level_filter())
        .with_module_level("pg_srv", log_level.to_level_filter());

    // console_subscriber::init();

    log::set_boxed_logger(Box::new(logger)).unwrap();
    ReportingLogger::init(Box::new(LocalReporter::new()), log_level.to_level_filter()).unwrap();

    // metrics
    let registry = Registry::new();
    // pass metrics registry to config
    let config = Config::default().with_registry(registry.clone());

    
    // Start the metrics server
    let metrics_addr: SocketAddr = "0.0.0.0:9100".parse().unwrap();
    tokio::spawn(run_metrics_server(registry.clone(), metrics_addr));
    // start cubesql
    let runtime = Builder::new_multi_thread().enable_all().build().unwrap();
    println!("Initializing runtime");
    println!("Configuring CubeSQL");
    println!("Starting CubeSQL services");
    println!("Waiting for Ctrl+C signal to initiate shutdown");
    runtime.block_on(async move {
        config.configure().await;
        let services = Arc::new(config.cube_services().await);
        log::debug!("Cube SQL Start");
        stop_on_ctrl_c(&services).await;
        services.wait_processing_loops().await.unwrap();
    });
}

async fn stop_on_ctrl_c(s: &Arc<CubeServices>) {
    let s = s.clone();
    tokio::spawn(async move {
        let mut counter = 0;
        loop {
            if let Err(e) = tokio::signal::ctrl_c().await {
                log::error!("Failed to listen for Ctrl+C: {}", e);
                break;
            }
            counter += 1;
            if counter == 1 {
                log::info!("Received Ctrl+C, shutting down.");
                s.stop_processing_loops(ShutdownMode::Fast).await.ok();
            } else if counter == 3 {
                log::info!("Received Ctrl+C 3 times, exiting immediately.");
                std::process::exit(130); // 130 is the default exit code when killed by a signal.
            }
        }
    });
}
// metrics handler
async fn metrics_handler(registry: Registry) -> Result<Response<Body>, hyper::Error> {
    let encoder = TextEncoder::new();
    let metric_families = registry.gather();
    let mut buffer = vec![];
    encoder.encode(&metric_families, &mut buffer).unwrap();
    Ok(Response::builder()
        .status(200)
        .header("Content-Type", encoder.format_type())
        .body(Body::from(buffer))
        .unwrap())
}
// run metrics server
async fn run_metrics_server(registry: Registry, addr: std::net::SocketAddr) {
    let metrics_service = make_service_fn(move |_| {
        let registry = registry.clone();
        async move {
            Ok::<_, hyper::Error>(service_fn(move |_| metrics_handler(registry.clone())))
        }
    });

    let server = Server::bind(&addr).serve(metrics_service);
    if let Err(e) = server.await {
        eprintln!("Metrics server error: {}", e);
    }
}
