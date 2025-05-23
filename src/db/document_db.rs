// src/db/document_db.rs
use elasticsearch::Elasticsearch;
use anyhow::Result;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct DocumentDbClient {
    // Odstránenie dead_code warningov: prefixovanie nepoužitých polí podčiarknutím
    pub _client: Arc<Elasticsearch>,
}

impl DocumentDbClient {
    pub async fn connect(url: &str) -> Result<Self> {
        let client = Elasticsearch::new(elasticsearch::http::transport::Transport::single_node(url)?);
        Ok(Self { _client: Arc::new(client) })
    }
    // TODO: Pridať ďalšie metódy podľa špecifikácie
}
