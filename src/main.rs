use crate::app::App;

mod app;
mod models;
mod routes;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    App::start().await
}
