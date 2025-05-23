// src/main.rs
mod setup;
mod db;
mod ai;
mod export;
mod utils;
mod apikey_broker;
mod config;
mod registries;

use clap::Parser;
use tracing_subscriber;
use std::sync::Arc;
use anyhow::Result;
use tokio::sync::mpsc;
use serde_json::json;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    #[arg(long)]
    pub query: String,
    #[arg(long)]
    pub category: Option<String>,
}

async fn run_osint_queries_for_target(
    target: &str,
    api_broker: Arc<apikey_broker::ApiKeyBroker<'static>>,
    db_graph: Arc<db::graph_db::GraphDbClient>,
    db_doc: Arc<db::document_db::DocumentDbClient>,
    ai_config: &config::AiConfig,
) -> anyhow::Result<()> {
    let (tx, mut rx) = mpsc::channel(16);
    for reg in registries::ALL_REGISTRIES.iter() {
        let tx = tx.clone();
        let reg = reg.clone();
        let api_broker = api_broker.clone();
        let target = target.to_string();
        tokio::spawn(async move {
            let result = reg.query(&target, &api_broker, None).await;
            let _ = tx.send(result).await;
        });
    }
    drop(tx);
    let mut all_results = vec![];
    while let Some(res) = rx.recv().await {
        match res {
            Ok(output) => {
                // TODO: Uložiť do Elasticsearch a Neo4j
                all_results.push(json!(output));
            },
            Err(e) => {
                tracing::error!(?e, "Chyba pri dotazovaní registru");
            }
        }
    }
    // AI sumarizácia
    let ai_summary = ai::analyze_text_onnx_or_pyo3(
        &utils::extract_text_for_ai(&json!(all_results)).unwrap_or_default(),
        ai_config,
        None
    ).await?;
    tracing::info!(?ai_summary, "AI sumarizácia výsledkov");
    // Export výsledkov
    export::export_results_to_json(&json!(all_results), "osint_results.json").await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let args = CliArgs::parse();
    let mut config = config::AppConfig::load_config("config.toml").await?;
    setup::ensure_api_keys_interactive(&mut config).await?;
    let api_broker = Arc::new(apikey_broker::ApiKeyBroker::new(Box::leak(Box::new(config.api_keys.clone()))));
    // Odstránenie dead_code warningov: prefixovanie nepoužitých premenných podčiarknutím
    let _db_graph: Arc<db::graph_db::GraphDbClient>;
    let _db_doc: Arc<db::document_db::DocumentDbClient>;
    _db_graph = Arc::new(db::graph_db::GraphDbClient::connect(
        config.db.neo4j_url.as_deref().unwrap_or("bolt://localhost:7687"),
        std::env::var("NEO4J_USER").unwrap_or_else(|_| "neo4j".to_string()).as_str(),
        std::env::var("NEO4J_PASS").unwrap_or_else(|_| "".to_string()).as_str(),
    ).await?);
    _db_doc = Arc::new(db::document_db::DocumentDbClient::connect(
        config.db.elasticsearch_url.as_deref().unwrap_or("http://localhost:9200")
    ).await?);
    run_osint_queries_for_target(&args.query, api_broker, _db_graph, _db_doc, &config.ai).await?;
    Ok(())
}
