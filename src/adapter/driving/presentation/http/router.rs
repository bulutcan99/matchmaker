use axum::Router;

pub fn router() -> Router {
    Router::new().merge(public()).nest("/api", protected())
}

pub fn public() -> Router {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
}

pub fn protected() -> Router {
    Router::new()
        .route("/me", get(me))
        .layer(axum::middleware::from_fn(auth::middleware::auth_middleware))
}
