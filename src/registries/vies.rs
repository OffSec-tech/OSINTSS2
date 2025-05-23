// src/registries/vies.rs
use super::*;
use serde_json::json;
use crate::apikey_broker::ApiKeyBroker;
use anyhow::Result;
use async_trait::async_trait;

#[derive(Debug, Default)]
pub struct ViesEuropa;

impl ViesEuropa {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl OsintRegistry for ViesEuropa {
    fn name(&self) -> &'static str {
        "VIES Europa"
    }
    fn type_id(&self) -> RegistryType {
        RegistryType::EU_VIES
    }
    async fn query(&self, target: &str, _api_keys_broker: &ApiKeyBroker, _additional_params: Option<&serde_json::Value>) -> Result<OsintOutput> {
        // TODO: Implementovať reálnu logiku dotazu na VIES
        Ok(OsintOutput::success(
            self.name().to_string(),
            self.type_id(),
            target.to_string(),
            json!({"dummy": "result"})
        ))
    }
    fn get_api_key_requirements(&self) -> Vec<ApiKeyRequirement> {
        vec![]
    }
}
