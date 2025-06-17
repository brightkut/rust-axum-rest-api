use axum::{http::Request, middleware::Next, response::Response};
use tower_cookies::Cookies;

use crate::{Error, Result, web::ACCESS_TOKEN};

pub async fn mw_require_auth<B>(
    cookies: Cookies,
    req: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    let access_token = cookies.get(ACCESS_TOKEN).map(|c| c.value().to_string());

    access_token.ok_or(Error::InvalidAccessToken)?;

    Ok(next.run(req).await)
}
