mod expense;

use axum::body::Body;
use axum::http::{Response, StatusCode};
use axum::serve;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::boxed;
use std::env::var;
use crate::expense::get_expense_json;

use axum::{response::IntoResponse, routing::get, Router};
use clap::Parser;
use std::net::{IpAddr, Ipv6Addr, SocketAddr};
use std::str::FromStr;
use tower::{ServiceBuilder, ServiceExt};
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

// Setup the command line interface with clap.
#[derive(Parser, Debug)]
#[clap(name = "server", about = "A server for our wasm project!")]
struct Opt {
    /// set the log level
    #[clap(short = 'l', long = "log", default_value = "debug")]
    log_level: String,

    /// set the listen addr
    #[clap(short = 'a', long = "addr", default_value = "::1")]
    addr: String,

    /// set the listen port
    #[clap(short = 'p', long = "port", default_value = "8080")]
    port: u16,

    /// set the directory where static files are to be found
    #[clap(long = "static-dir", default_value = "./dist")]
    static_dir: String,
}

#[derive(Clone)]
struct State {pool: Pool<Postgres>}

#[tokio::main]
async fn main() -> anyhow::Result<()>{
    let opt = Opt::parse();

    // Setup logging & RUST_LOG from args
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", format!("{},hyper=info,mio=info", opt.log_level))
    }
    // enable console logging
    tracing_subscriber::fmt::init();

    let serve_dir = ServeDir::new(opt.static_dir);

    let db_url = var("DATABASE_URL").unwrap();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    let state = State {pool: pool};

    let app = Router::new()
        .route("/api/hello", get(hello))
        .route("/api/expense/:id", get(get_expense_json))
        .fallback_service(serve_dir)
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
        .with_state(state);

    let sock_addr = SocketAddr::from((
        IpAddr::from_str(opt.addr.as_str()).unwrap_or(IpAddr::V6(Ipv6Addr::LOCALHOST)),
        opt.port,
    ));

    log::info!("listening on http://{}", sock_addr);

    let listener = tokio::net::TcpListener::bind(sock_addr).await.unwrap();
    serve(listener, app).await.unwrap();
    Ok(())
}

async fn hello() -> impl IntoResponse {
    "hello from server!"
}
