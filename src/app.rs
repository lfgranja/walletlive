use axum::{Router, routing::get};
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::{
    Layer, fmt::format::FmtSpan, layer::SubscriberExt, util::SubscriberInitExt,
};

pub struct App;

impl App {
    pub async fn start() -> color_eyre::Result<()> {
        let layer = tracing_subscriber::fmt::layer()
            .with_span_events(FmtSpan::NEW)
            .boxed();

        tracing_subscriber::registry().with(layer).init();

        let listener = TcpListener::bind("0.0.0.0:3000").await?;
        let router = Router::new().route("/", get(hello_world));

        info!("Starting service");

        axum::serve(listener, router).await?;

        Ok(())
    }
}

#[tracing::instrument]
async fn hello_world() -> &'static str {
    "hello, world!"
}
