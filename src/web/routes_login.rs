use axum::{Json, Router, routing::post};
use serde::Deserialize;
use serde_json::{Value, json};
use tower_cookies::{Cookie, Cookies};

use crate::{Error, Result, web::ACCESS_TOKEN};

#[derive(Debug, Deserialize)]
struct LoginDto {
    username: String,
    password: String,
}

pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}

async fn api_login(cookies: Cookies, payload: Json<LoginDto>) -> Result<Json<Value>> {
    println!("Request to Login API");

    if payload.username != "admin" || payload.password != "admin" {
        return Err(Error::LoginFail);
    }

    // FIXME:   Implement real access token logic
    cookies.add(Cookie::new(ACCESS_TOKEN, "user-1.exp.sign"));

    let body = Json(json!({
        "message": "Login success"
    }));

    Ok(body)
}
