use askama::Template;
use axum::{
    Form, Router,
    response::{Html, IntoResponse, Redirect},
    routing::get,
};
use serde::Deserialize;

use crate::{
    app::AppState, auth::user::UnauthenticatedUser, error::AppError, repository::Repository,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(index))
        .route("/login", get(login_page).post(login))
}

#[derive(Template)]
#[template(path = "login.html")]
struct LoginPage;

async fn login_page() -> Result<Html<String>, AppError> {
    let html = LoginPage.render()?;
    Ok(Html(html))
}

#[derive(Deserialize)]
struct LoginForm {
    username: String,
    password: String,
}

async fn login(
    repository: Repository,
    Form(request): Form<LoginForm>,
) -> Result<impl IntoResponse, AppError> {
    let unauth_user = UnauthenticatedUser::new(request.username, request.password);
    let user = match unauth_user.authenticate(&repository).await {
        Ok(user) => user,
        Err(AppError::UserDoesNotExist) => unauth_user.register(&repository).await?,
        Err(other_err) => return Err(other_err),
    };

    Ok(Redirect::to("/"))
}

async fn index() -> Result<Html<String>, AppError> {
    Ok(Html("Hello, World!".to_string()))
}
