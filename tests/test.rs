use anyhow::{Ok, Result};
use serde_json::json;

#[tokio::test]
async fn routes_tests() -> Result<()> {
    let http_client = httpc_test::new_client("http://localhost:8080")?;

    // Test hello handler without params
    // let _ = http_client.do_get("/hello").await?.print().await;

    // Test hello handler without params
    // let _ = http_client.do_get("/hello?name=Boby").await?.print().await;

    // Test hello2 handler with path
    // let _ = http_client.do_get("/hello2/Boby").await?.print().await;

    // Test no routes handler
    // let _ = http_client.do_get("/src/main.rs").await?.print().await;
    //
    // Test login handler
    let req_login = http_client.do_post(
        "/api/login",
        json!({
            "username": "admin",
            "password": "admin"
        }),
    );

    req_login.await?.print().await?;

    let req_create_ticket = http_client.do_post(
        "/api/tickets",
        json!({
            "title": "Mikut concert"
        }),
    );

    req_create_ticket.await?.print().await?;

    let _ = http_client.do_get("/api/tickets").await?.print().await?;

    Ok(())
}
