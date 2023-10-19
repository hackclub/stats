use ingest::elastic::chronicle_elastic_client;

#[tokio::test]
async fn ping() -> Result<(), Box<dyn std::error::Error>> {
    let client = chronicle_elastic_client()?;
    assert!(client.ping().send().await?.status_code().is_success());
    Ok(())
}
