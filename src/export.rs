// src/export.rs
use anyhow::Result;

pub async fn export_results_to_json(_results: &serde_json::Value, _path: &str) -> Result<()> {
    // TODO: Implementovať export do JSON
    Ok(())
}

// Odstránenie dead_code warningov: prefixovanie nepoužitých funkcií podčiarknutím
pub async fn _export_results_to_csv(_results: &serde_json::Value) {}
pub async fn _export_results_to_xml(_results: &serde_json::Value) {}
