use axum::{
    RequestPartsExt, async_trait,
    extract::{FromRequestParts, State},
    http::{Request, request::Parts},
    middleware::Next,
    response::Response,
};
use tower_cookies::{Cookie, Cookies};

use crate::{Error, Result, ctx::Ctx, web::ACCESS_TOKEN};

pub async fn mw_ctx_resolver<B>(
    // _mc: State<TicketController> If we want to have db connection for validate token
    cookies: Cookies,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    println!("Log in Context resolver middleware");

    let access_token = cookies.get(ACCESS_TOKEN).map(|c| c.value().to_string());

    let result_ctx = match access_token.ok_or(Error::NoAuthTokenInCookie) {
        Ok(_) => Ok(Ctx::new(12345)),
        Err(e) => Err(e),
    };

    if result_ctx.is_err() && !matches!(result_ctx, Err(Error::NoAuthTokenInCookie)) {
        cookies.remove(Cookie::named(ACCESS_TOKEN));
    }

    req.extensions_mut().insert(result_ctx);

    Ok(next.run(req).await)
}

pub async fn mw_require_auth<B>(
    ctx: Result<Ctx>,
    req: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    println!("Log in Auth middleware");

    // this function will call from_request_parts to get context
    ctx?;

    Ok(next.run(req).await)
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        println!("Context Extractor");

        // // Logic for extract token from cookie and validate logic
        // let cookies = parts.extract::<Cookies>().await.unwrap();
        //
        // let access_token = cookies.get(ACCESS_TOKEN).map(|c| c.value().to_string());
        //
        // access_token.ok_or(Error::InvalidAccessToken)?;
        //
        // // hardcode user id
        // Ok(Ctx::new(12345))
        //

        // Get context from extension
        parts
            .extensions
            .get::<Result<Ctx>>()
            .ok_or(Error::AuthFailedCtxNotInReq)?
            .clone()
    }
}
