use askama::Template;
use axum::{Router, response::Html, routing::get};

use crate::{app::AppState, error::AppError};

pub fn router() -> Router<AppState> {
    Router::new().route("/login", get(login_page))
}

#[derive(Template)]
#[template(path = "login.html")]
struct LoginPage;

async fn login_page() -> Result<Html<String>, AppError> {
    let html = LoginPage.render()?;
    Ok(Html(html))
}
