// src/registries/mod.rs
use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use crate::apikey_broker::ApiKeyBroker;
use anyhow::Result;
use std::fmt::Debug;
use std::sync::Arc;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum RegistryType {
    SK_ORSR, SK_ZR, EU_VIES, WHOIS,
    SocialMediaProfile, SocialMediaPost, DataBreachEntry, PassiveDnsRecords,
    SubdomainList, SslCertificateDetails, IpGeolocationInfo, IpReputationDetails,
    CompanyInfoOpenCorp, SanctionScreeningResult, PasteSiteMention,
    GoogleSearchResult, ShodanHostInfo, WaybackMachineSnapshot,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyRequirement {
    pub service_name: String,
    pub key_name_in_config: String,
    pub registration_url: String,
    pub instructions: String,
    pub is_optional: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OsintOutput {
    pub source_registry_name: String,
    pub source_registry_type: RegistryType,
    pub query_input: String,
    pub data: Value,
    pub error_message: Option<String>,
    pub success: bool,
    #[serde(default = "Utc::now")]
    pub timestamp: DateTime<Utc>,
}

impl Default for OsintOutput {
    fn default() -> Self {
        Self {
            source_registry_name: String::new(),
            source_registry_type: RegistryType::SK_ORSR,
            query_input: String::new(),
            data: serde_json::Value::Null,
            error_message: None,
            success: false,
            timestamp: Utc::now(),
        }
    }
}

impl OsintOutput {
    pub fn success(source_registry_name: String, source_registry_type: RegistryType, query_input: String, data: Value) -> Self {
        Self { source_registry_name, source_registry_type, query_input, data, error_message: None, success: true, timestamp: Utc::now() }
    }

    pub fn _error(source_registry_name: String, source_registry_type: RegistryType, error: String) -> Self {
        Self {
            source_registry_name,
            source_registry_type,
            query_input: String::new(),
            data: serde_json::Value::Null,
            error_message: Some(error),
            success: false,
            timestamp: Utc::now(),
        }
    }
}

#[async_trait]
pub trait OsintRegistry: Debug + Send + Sync {
    fn name(&self) -> &'static str;
    fn type_id(&self) -> RegistryType;
    async fn query(&self, target: &str, api_keys_broker: &ApiKeyBroker, additional_params: Option<&Value>) -> Result<OsintOutput>;
    fn get_api_key_requirements(&self) -> Vec<ApiKeyRequirement>;
}

// Deklarácie existujúcich a nových modulov
pub mod orsr;
pub mod zr;
pub mod vies;
pub mod whois;
// Odstránené: pub mod ebr; pub mod eurojust_test;

pub mod social_media;
pub mod data_breaches;
pub mod network_intelligence;
pub mod corporate_records;
pub mod threat_intelligence;
pub mod web_archives;
pub mod google_search;

lazy_static::lazy_static! {
    pub static ref ALL_REGISTRIES: Vec<Arc<dyn OsintRegistry>> = {
        let mut registries: Vec<Arc<dyn OsintRegistry>> = Vec::new();
        registries.push(Arc::new(orsr::OrsrSlovensko::new()));
        registries.push(Arc::new(zr::ZrSlovensko::new()));
        registries.push(Arc::new(vies::ViesEuropa::new()));
        registries.push(Arc::new(whois::WhoisLookup::new()));
        registries.push(Arc::new(google_search::GoogleSearch::new()));
        registries.push(Arc::new(social_media::TwitterUserSearch::new()));
        registries.push(Arc::new(data_breaches::HibpEmailCheck::new()));
        registries.push(Arc::new(network_intelligence::ShodanHostInfo::new()));
        registries.push(Arc::new(web_archives::WaybackMachineSnapshot::new()));
        registries.push(Arc::new(corporate_records::CompanyInfoOpenCorp::new()));
        registries.push(Arc::new(threat_intelligence::PasteSiteMention::new()));
        registries
    };
}
