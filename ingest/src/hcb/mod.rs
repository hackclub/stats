pub mod event;
pub mod session;
pub mod transaction;
pub mod user;

use elasticsearch::http::request::JsonBody;
use elasticsearch::{BulkParts, Elasticsearch, SearchParts};
use fake::{Dummy, Fake, Faker};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{json, Value};
use std::any::type_name;
use std::collections::HashMap;

static PAGINATED_MODELS: [(&str, &str); 1] = [("CanonicalTransaction", "search-hcb-transactions")];

#[async_trait::async_trait]
pub trait HcbModel {
    async fn idx<T: Serialize + DeserializeOwned + std::marker::Send + std::fmt::Debug>(
        client: &Elasticsearch,
        index_suffix: &str,
        egestion_endpoint: &str,
        pb: ProgressBar,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let index = format!("search-hcb-{index_suffix}");
        let index = index.as_str();

        pb.set_message(format!("Clearing {index}..."));
        clear_index(client, index).await?;
        pb.inc(1);

        // Get the egestion from the source
        pb.set_message(format!("Ingesting {index_suffix}..."));
        let url = url::Url::parse(EGESTION_ROOT)
            .unwrap()
            .join(egestion_endpoint)
            .unwrap();
        let objects = Self::ingest_model_data::<T>(client, &url.to_string()).await?;
        pb.inc(1);

        // Parse & format it ready for ES ingestion
        pb.set_message(format!("Parsing {index_suffix}..."));
        let objects_body: Vec<JsonBody<Value>> = objects
            .into_iter()
            .flat_map(|obj| vec![json!({ "index": { } }).into(), json!(obj).into()])
            .collect();
        pb.inc(1);

        pb.set_message(format!("Indexing {index}..."));
        let response = client
            .bulk(BulkParts::Index(index))
            .body(objects_body)
            .send()
            .await?
            .text()
            .await?;
        pb.finish_with_message(format!("✅ {index}"));

        Ok(())
    }

    async fn ingest_model_data<
        T: Serialize + DeserializeOwned + std::marker::Send + std::fmt::Debug,
    >(
        client: &Elasticsearch,
        egestion_endpoint: &str,
    ) -> Result<Vec<T>, Box<dyn std::error::Error>> {
        /*  Some tables are too big to index all in one go.
            Hence, we need to paginate
            The HCB transaction egestion enpoint, for example,
            orders by id and has an "after" parameter.
            It also has a "count" parameter, which is the
            number of rows to return and defaults to 1_000.
        */
        let mut query: HashMap<String, String> = HashMap::new();

        for model in PAGINATED_MODELS {
            if type_name::<T>().ends_with(format!("::{}", model.0).as_str()) {
                // let es_response_raw = client
                //     .search(SearchParts::Index(&[model.1]))
                //     .body(json!({
                //         "query": { "match_all": {} },
                //         "size": 1,
                //         "sort": [{ "id": { "order": "asc" } }],
                //         "_source": ["id"],
                //     }))
                //     .send()
                //     .await?
                //     .text()
                //     .await?;
                // let es_res = serde_json::from_str::<Value>(&es_response_raw)?;
                // println!("{:#?}", es_res);

                // let top_id = es_res.get("hits").unwrap().get("hits").unwrap()[0]
                //     .get("_source")
                //     .unwrap()
                //     .get("id")
                //     .unwrap();
                let top_id = -1;

                query.insert("after".to_string(), top_id.to_string());
            }
        }

        // Get the egestion from the source
        let objects_raw = reqwest::get(Self::egestion_url(egestion_endpoint, query))
            .await?
            .text()
            .await?;

        let objects: Vec<T> = serde_json::from_str(&objects_raw)?;

        Ok(objects)
    }

    fn egestion_url(egestion_endpoint: &str, queries: HashMap<String, String>) -> url::Url {
        let mut url = url::Url::parse(EGESTION_ROOT)
            .expect("a valid egestion url")
            .join(egestion_endpoint)
            .expect("a valid joined egestion url");

        for (key, value) in queries {
            url.query_pairs_mut().append_pair(&key, &value);
        }

        url
    }

    fn fake() -> Self
    where
        Self: Dummy<Faker>,
    {
        Faker.fake::<Self>()
    }
}

static EGESTION_ROOT: &str = "http://localhost:3000/api/egest/";

pub async fn recreate_index(
    _client: &elasticsearch::Elasticsearch,
    index: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    todo!("Delete then create index {}", index);
}

pub async fn clear_index(
    client: &elasticsearch::Elasticsearch,
    index: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let slice = &[index];
    let parts = elasticsearch::DeleteByQueryParts::Index(slice);
    elasticsearch::DeleteByQuery::<elasticsearch::http::request::JsonBody<&str>>::new(
        client.transport(),
        parts,
    )
    .analyze_wildcard(true)
    .q("*:*")
    .send()
    .await?;

    Ok(())
}
