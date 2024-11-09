use crate::Result;

async fn get_repo_info() -> Result<serde_json::Value> {
    let response = crate::client::github()
        .path("repos")
        .arg("os-checker")
        .arg("os-checker")
        .send()
        .await?;
    crate::parse_response(response).await
}

#[tokio::test]
async fn os_checker() -> Result<()> {
    let json = get_repo_info().await?;
    dbg!(json);
    Ok(())
}
