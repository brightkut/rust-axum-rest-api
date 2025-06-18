use std::net::SocketAddr;

use axum::{
    Router,
    extract::{Path, Query},
    http::Request,
    middleware,
    response::{Html, IntoResponse, Response},
    routing::{get, get_service},
};
use serde::Deserialize;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

use crate::{model::TicketController, web::routes_login::routes};

pub use self::error::{Error, Result};

mod ctx;
mod error;
mod model;
mod web;

#[tokio::main]
async fn main() -> Result<()> {
    let tc = TicketController::new().await?;

    // apply auth middleware for ticket handler only
    let routes_internal = web::routes_ticket::routes(tc.clone())
        .route_layer(middleware::from_fn(web::middleware_auth::mw_require_auth));

    let route_all = Router::new()
        .merge(routes_hello())
        .merge(routes())
        // set prefix path for this route
        .nest("/api", routes_internal)
        .layer(middleware::map_request(main_request_middleware))
        .layer(middleware::map_response(main_response_middleware))
        .layer(CookieManagerLayer::new())
        .layer(middleware::from_fn_with_state(
            tc.clone(),
            web::middleware_auth::mw_ctx_resolver,
        ))
        // this fallback_service use when no route to go it will go to this function
        .fallback_service(routes_static());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    println!("Server start on {addr}");
    axum::Server::bind(&addr)
        .serve(route_all.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn main_request_middleware<T>(req: Request<T>) -> Request<T> {
    println!("Main request middleware");
    req
}

async fn main_response_middleware(res: Response) -> Response {
    println!("Main response middleware");
    println!();

    res
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/:name", get(handler_hello2))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("Request to: Hello Handler");

    let name = params.name.as_deref().unwrap_or("no params");

    Html(format!("<strong>Hello World with params: {name}</strong>"))
}

async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("Request to: Hello2 Handler");
    Html(format!("<strong>Hello World with path: {name}</strong>"))
}
