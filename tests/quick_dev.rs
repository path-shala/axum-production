use anyhow::{Ok, Result};

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://127.0.0.1:9090")?;
    hc.do_get("/hello2/subhrajit").await?.print().await?;
    Ok(())
}
