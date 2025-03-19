#![allow(unused)]

use anyhow::Result;

const BASE_URL: &str = "http://localhost:8080";

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client(BASE_URL)?;

    hc.do_get("/hello?name=Bob").await?.print().await?;

    Ok(())
}
