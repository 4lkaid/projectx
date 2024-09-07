use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let _worker_guard = projectx::run().await?;
    Ok(())
}
