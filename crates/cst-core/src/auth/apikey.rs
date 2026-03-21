//! API key authentication — multi-key pool with Keychain storage.

use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};

use super::EnvMap;

const SERVICE_NAME: &str = "claude-sentinel";

/// A single API key entry in the pool.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyEntry {
    /// Slot number (1-based). Determines priority order.
    pub slot: u8,
    /// Keychain account name used to retrieve the key.
    pub keychain_account: String,
    /// Human note about this key.
    #[serde(default)]
    pub note: String,
}

/// The API key pool stored in `auth/api_keys.toml`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApiKeyPool {
    pub keys: Vec<ApiKeyEntry>,
}

impl ApiKeyPool {
    /// Store a new API key in the OS Keychain and add it to the pool.
    pub fn add_key(&mut self, profile_name: &str, slot: u8, api_key: &str, note: &str) -> Result<()> {
        let account = format!("{profile_name}-slot{slot}");
        store_in_keychain(&account, api_key)?;
        // Remove existing entry for this slot if any
        self.keys.retain(|k| k.slot != slot);
        self.keys.push(ApiKeyEntry {
            slot,
            keychain_account: account,
            note: note.to_string(),
        });
        self.keys.sort_by_key(|k| k.slot);
        Ok(())
    }

    /// Remove a key slot.
    pub fn remove_key(&mut self, slot: u8) -> Result<()> {
        let entry = self.keys.iter().find(|k| k.slot == slot)
            .ok_or_else(|| anyhow::anyhow!("slot {slot} not found"))?;
        delete_from_keychain(&entry.keychain_account)?;
        self.keys.retain(|k| k.slot != slot);
        Ok(())
    }

    /// Retrieve the API key for a given slot from the Keychain.
    pub fn retrieve_key(&self, slot: u8) -> Result<String> {
        let entry = self.keys.iter().find(|k| k.slot == slot)
            .ok_or_else(|| anyhow::anyhow!("slot {slot} not found in key pool"))?;
        retrieve_from_keychain(&entry.keychain_account)
    }

    /// Get the highest-priority valid key's env vars.
    pub fn env_vars_for_slot(&self, slot: u8) -> Result<EnvMap> {
        let key = self.retrieve_key(slot)?;
        let mut map = EnvMap::new();
        map.insert("ANTHROPIC_API_KEY".to_string(), key);
        Ok(map)
    }

    /// Return slots sorted by priority.
    pub fn sorted_slots(&self) -> Vec<u8> {
        let mut slots: Vec<u8> = self.keys.iter().map(|k| k.slot).collect();
        slots.sort();
        slots
    }

    pub fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }
}

fn store_in_keychain(account: &str, secret: &str) -> Result<()> {
    let entry = keyring::Entry::new(SERVICE_NAME, account)
        .context("creating keychain entry")?;
    entry.set_password(secret)
        .context("storing key in keychain")?;
    Ok(())
}

fn retrieve_from_keychain(account: &str) -> Result<String> {
    let entry = keyring::Entry::new(SERVICE_NAME, account)
        .context("creating keychain entry")?;
    entry.get_password()
        .context("retrieving key from keychain — run `cst add-key` to add credentials")
}

fn delete_from_keychain(account: &str) -> Result<()> {
    let entry = keyring::Entry::new(SERVICE_NAME, account)
        .context("creating keychain entry")?;
    // Ignore error if entry doesn't exist
    let _ = entry.delete_credential();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_key_pool_add_and_sorted_slots() {
        let mut pool = ApiKeyPool::default();
        // We can't actually write to keychain in tests, so test the pool struct
        pool.keys.push(ApiKeyEntry { slot: 2, keychain_account: "p-slot2".into(), note: "".into() });
        pool.keys.push(ApiKeyEntry { slot: 1, keychain_account: "p-slot1".into(), note: "".into() });
        let slots = pool.sorted_slots();
        assert_eq!(slots, vec![1, 2]);
    }

    #[test]
    fn test_api_key_pool_is_empty() {
        let pool = ApiKeyPool::default();
        assert!(pool.is_empty());
    }

    #[test]
    fn test_api_key_pool_remove_slot() {
        let mut pool = ApiKeyPool {
            keys: vec![
                ApiKeyEntry { slot: 1, keychain_account: "p-slot1".into(), note: "".into() },
                ApiKeyEntry { slot: 2, keychain_account: "p-slot2".into(), note: "".into() },
            ],
        };
        pool.keys.retain(|k| k.slot != 1);
        assert_eq!(pool.sorted_slots(), vec![2]);
    }
}
