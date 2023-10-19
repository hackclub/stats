use elasticsearch::{
    auth::Credentials,
    cert::CertificateValidation,
    http::transport::{BuildError, SingleNodeConnectionPool, TransportBuilder},
    Elasticsearch,
};
use std::env;

pub fn chronicle_elastic_client() -> Result<Elasticsearch, BuildError> {
    let url = env::var("CHRONICLE_ELASTIC_URL").expect("a chronicle url present");
    let url = url::Url::parse(&url).expect("a valid chronicle url");

    let key_id = env::var("CHRONICLE_ELASTIC_KEY_ID").expect("a chronicle key ID present");
    let key_secret = env::var("CHRONICLE_ELASTIC_KEY_SECRET").expect("a chronicle key secret present");

    let conn_pool = SingleNodeConnectionPool::new(url);
    let credentials = Credentials::ApiKey(key_id, key_secret);
    let transport = TransportBuilder::new(conn_pool)
        .cert_validation(CertificateValidation::None)
        .auth(credentials)
        .build()?;

    Ok(Elasticsearch::new(transport))
}
