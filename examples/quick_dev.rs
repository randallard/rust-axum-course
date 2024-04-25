#![allow(unused)]

use anyhow::Result;
use axum::response::IntoResponse;

#[tokio::main]
async fn main() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    hc.do_get("/hello").await?.print().await?;
    hc.do_get("/hello?name=Ryan").await?.print().await?;
    hc.do_get("/hello2/Ryan").await?.print().await?;
    hc.do_get("/public/index.htm").await?.print().await?;
    hc.do_get("/src/main.rs").await?.print().await?;
    
    Ok(())
}