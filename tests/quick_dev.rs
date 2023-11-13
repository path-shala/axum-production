use anyhow::{Ok, Result};
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://127.0.0.1:9090")?;
    hc.do_get("/hello2/subhrajit").await?.print().await?;
    hc.do_get("/hello").await?.print().await?;
    // hc.do_get("/src/main.rs").await?.print().await?;
    let req_login = hc.do_post(
        "/api/login",
        json!({
    "username": "JohnDoe",
    "password": "abcd123"}),
    );
    req_login.await?.print().await?;
    let ticket = hc.do_post(
        "/api/tickets",
        json!({
            "title": "test ticket",
        }),
    );
    ticket.await?.print().await?;
    hc.do_get("/api/tickets").await?.print().await?;
    hc.do_delete("/api/tickets/1").await?.print().await?;
    Ok(())
}
