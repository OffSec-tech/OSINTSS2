// src/registries/data_breaches.rs
use super::*;
use serde_json::json;
use crate::apikey_broker::ApiKeyBroker;
use anyhow::Result;
use async_trait::async_trait;

/// Reprezentuje OSINT register pre kontrolu emailu v databáze únikov (HIBP).
#[derive(Debug, Default)]
pub struct HibpEmailCheck;

impl HibpEmailCheck {
    /// Vytvorí novú inštanciu.
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl OsintRegistry for HibpEmailCheck {
    /// Názov modulu.
    fn name(&self) -> &'static str {
        "HaveIBeenPwned Email Check"
    }
    /// Typ registru.
    fn type_id(&self) -> RegistryType {
        RegistryType::DataBreachEntry
    }
    /// Spustí dotaz na HIBP (TODO: implementovať reálnu logiku).
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
