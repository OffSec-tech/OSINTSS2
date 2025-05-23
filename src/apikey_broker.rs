// src/apikey_broker.rs
use crate::config::ApiKeys;

#[derive(Debug, Clone)]
pub struct ApiKeyBroker<'a> {
    // Odstránenie dead_code warningov: prefixovanie nepoužitých polí podčiarknutím
    pub _keys: &'a ApiKeys,
}

impl<'a> ApiKeyBroker<'a> {
    pub fn new(keys: &'a ApiKeys) -> Self {
        Self { _keys: keys }
    }
    // Odstránenie dead_code warningov: prefixovanie nepoužitých metód podčiarknutím
    pub fn _get_key(&self, _service: &str) -> Option<&String> { None }

    pub fn get_key(&self, service: &str) -> Option<&str> {
        match service {
            "orsr" => self._keys.orsr.as_deref(),
            "zr" => self._keys.zr.as_deref(),
            "vies" => self._keys.vies.as_deref(),
            "whois" => self._keys.whois.as_deref(),
            _ => None,
        }
    }
}
