// src/registries/corporate_records.rs
use super::*;
use serde_json::json;
use crate::apikey_broker::ApiKeyBroker;
use anyhow::Result;
use async_trait::async_trait;

/// Reprezentuje OSINT register pre získavanie firemných informácií z OpenCorporates.
#[derive(Debug, Default)]
pub struct CompanyInfoOpenCorp;

impl CompanyInfoOpenCorp {
    /// Vytvorí novú inštanciu.
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl OsintRegistry for CompanyInfoOpenCorp {
    /// Názov modulu.
    fn name(&self) -> &'static str {
        "Company Info OpenCorporates"
    }
    /// Typ registru.
    fn type_id(&self) -> RegistryType {
        RegistryType::CompanyInfoOpenCorp
    }
    /// Spustí dotaz na OpenCorporates (TODO: implementovať reálnu logiku).
    async fn query(&self, target: &str, _api_keys_broker: &ApiKeyBroker, _additional_params: Option<&serde_json::Value>) -> Result<OsintOutput> {
        Ok(OsintOutput::success(
            self.name().to_string(),
            self.type_id(),
            target.to_string(),
            json!({"dummy": "result"})
        ))
    }
    /// Požiadavky na API kľúče (žiadne).
    fn get_api_key_requirements(&self) -> Vec<ApiKeyRequirement> {
        vec![]
    }
}
