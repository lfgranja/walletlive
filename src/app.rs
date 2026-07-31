use std::sync::Arc;

use crate::{models::Asset, routes};
use axum::Router;
use tokio::{net::TcpListener, sync::Mutex};
use tracing::info;
use tracing_subscriber::{
    Layer, fmt::format::FmtSpan, layer::SubscriberExt, util::SubscriberInitExt,
};

pub struct App;

#[derive(Clone)]
pub struct AppState {
    pub assets: Arc<Mutex<Vec<Asset>>>,
}

impl AppState {
    fn new() -> Self {
        Self {
            assets: Default::default(),
        }
    }
}

impl App {
    pub async fn start() -> color_eyre::Result<()> {
        let layer = tracing_subscriber::fmt::layer()
            .with_span_events(FmtSpan::NEW)
            .boxed();

        tracing_subscriber::registry().with(layer).init();

        let listener = TcpListener::bind("0.0.0.0:3000").await?;
        let router = Router::new()
            .nest("/api", routes::api::router())
            .with_state(AppState::new());

        info!("Starting service");

        axum::serve(listener, router).await?;

        Ok(())
    }
}
