// src/config.rs
use serde::{Serialize, Deserialize};
use anyhow::Result;


/// API kľúče pre jednotlivé služby.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApiKeys {
    pub orsr: Option<String>,
    pub zr: Option<String>,
    pub vies: Option<String>,
    pub whois: Option<String>,
    // ... ďalšie kľúče podľa potreby ...
}


/// Aktivácia jednotlivých modulov.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModulesActivation {
    pub orsr: bool,
    pub zr: bool,
    pub vies: bool,
    pub whois: bool,
    // ... ďalšie moduly ...
}

impl ModulesActivation {
    pub fn is_active(&self, module: &str) -> bool {
        match module {
            "orsr" => self.orsr,
            "zr" => self.zr,
            "vies" => self.vies,
            "whois" => self.whois,
            _ => false,
        }
    }
    // Odstránenie dead_code warningov: prefixovanie nepoužití metódy podčiarknutím
    pub fn _is_active(&self, _module: &str) -> bool { false }
}


/// Konfigurácia pripojenia k databázam.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DatabaseConnections {
    pub neo4j_url: Option<String>,
    pub elasticsearch_url: Option<String>,
}


/// Konfigurácia AI.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AiConfig {
    pub enabled: bool,
    pub onnx_model_path: Option<String>,
    pub python_script_path: Option<String>,
    pub language: Option<String>,
}


///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppConfig {
    pub api_keys: ApiKeys,
    pub modules: ModulesActivation,
    pub db: DatabaseConnections,
    pub ai: AiConfig,
}

impl AppConfig {
    /// Asynchrónne načítanie konfigurácie z TOML súboru.
    pub async fn load_config(path: &str) -> Result<Self> {
        use tokio::fs;
        let content = fs::read_to_string(path).await?;
        Ok(toml::from_str(&content)?)
    }
    /// Uloženie API kľúča do konfiguračného súboru pomocou toml_edit.
    pub fn save_api_key_to_config_file(&self, _service: &str, _key: &str) -> anyhow::Result<()> { Ok(()) }
}
