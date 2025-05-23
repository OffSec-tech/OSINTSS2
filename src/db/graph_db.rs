// src/db/graph_db.rs
use neo4rs::Graph;
use std::sync::Arc;

pub struct GraphDbClient {
    // Odstránenie dead_code warningov: prefixovanie nepoužitých polí podčiarknutím
    pub _client: Arc<Graph>,
}

impl GraphDbClient {
    pub async fn connect(url: &str, user: &str, password: &str) -> anyhow::Result<Self> {
        let client = Graph::new(url, user, password).await?;
        Ok(GraphDbClient {
            _client: Arc::new(client),
        })
    }
    // TODO: Pridať ďalšie metódy podľa špecifikácie
}
