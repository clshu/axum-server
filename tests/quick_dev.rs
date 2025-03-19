#![allow(unused)]

use anyhow::Result;
use serde_json::json;

const BASE_URL: &str = "http://localhost:8080";

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client(BASE_URL)?;

    hc.do_get("/hello2/Mike").await?.print().await?;

    let req_log = hc
        .do_post(
            "/api/login",
            json!({
                "email": "admin@test.com",
                "password": "admin",
            }),
        )
        .await?
        .print()
        .await?;

    let req_log = hc
        .do_post(
            "/api/login",
            json!({
                "email": "puppy@test.com",
                "password": "admin",
            }),
        )
        .await?
        .print()
        .await?;

    Ok(())
}
