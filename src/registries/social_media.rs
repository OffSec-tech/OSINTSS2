// src/registries/social_media.rs
use super::*;
use serde_json::json;
use crate::apikey_broker::ApiKeyBroker;
use anyhow::Result;
use async_trait::async_trait;

/// Reprezentuje OSINT register pre vyhľadávanie používateľov na Twitteri.
#[derive(Debug, Default)]
pub struct TwitterUserSearch;

impl TwitterUserSearch {
    /// Vytvorí novú inštanciu.
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl OsintRegistry for TwitterUserSearch {
    /// Názov modulu.
    fn name(&self) -> &'static str {
        "Twitter User Search"
    }
    /// Typ registru.
    fn type_id(&self) -> RegistryType {
        RegistryType::SocialMediaProfile
    }
    /// Spustí dotaz na Twitter (TODO: implementovať reálnu logiku).
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
