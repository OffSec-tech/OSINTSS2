// src/setup.rs
use crate::config::AppConfig;
use anyhow::Result;
use std::io::{self, BufReader};

pub async fn ensure_api_keys_interactive(config: &mut AppConfig) -> Result<()> {
    let changed = false;
    let stdin = BufReader::new(io::stdin());
    // Oprava: najprv si pripravíme nové hodnoty a až potom priradíme
    let mut new_api_keys = config.api_keys.clone();
    for (service, key_opt) in [
        ("orsr", &config.api_keys.orsr),
        ("zr", &config.api_keys.zr),
        ("vies", &config.api_keys.vies),
        ("whois", &config.api_keys.whois),
    ] {
        if key_opt.is_none() || key_opt.as_ref().unwrap().is_empty() {
            let key = rpassword::prompt_password(format!("Zadajte API kľúč pre {}: ", service)).unwrap();
            match service {
                "orsr" => new_api_keys.orsr = Some(key),
                "zr" => new_api_keys.zr = Some(key),
                "vies" => new_api_keys.vies = Some(key),
                "whois" => new_api_keys.whois = Some(key),
                _ => {}
            }
        }
    }
    config.api_keys = new_api_keys;
    tracing::info!("API kľúče boli aktualizované a uložené.");
    Ok(())
}
