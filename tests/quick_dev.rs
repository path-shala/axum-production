use anyhow::{Ok, Result};
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://127.0.0.1:9090")?;
    hc.do_get("/hello2/subhrajit").await?.print().await?;
    hc.do_get("/hello").await?.print().await?;
    hc.do_get("/src/main.rs").await?.print().await?;
    let req_login = hc.do_post(
        "/api/login",
        json!({
    "username": "JohnDoe",
    "password": "abcd1234"}),
    );
    req_login.await?.print().await?;
    Ok(())
}
