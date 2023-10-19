#![feature(const_type_name)]

use ingest::elastic::chronicle_elastic_client;
use ingest::index_all;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let client = chronicle_elastic_client()?;
    index_all(&client).await?;

    Ok(())
}

// docker compose exec db psql postgres://postgres:postgres@localhost:5432/bank_development

//docker-compose run web bash
// bundle exec rails c
