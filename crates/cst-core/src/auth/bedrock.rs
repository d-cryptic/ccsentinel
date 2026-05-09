//! AWS Bedrock authentication — inject AWS credential env vars.

use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::EnvMap;

/// AWS Bedrock configuration stored in `auth/aws.toml`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BedrockConfig {
    pub region: String,
    /// Keychain account name for ACCESS_KEY_ID.
    #[serde(default)]
    pub access_key_account: String,
    /// Keychain account name for SECRET_ACCESS_KEY.
    #[serde(default)]
    pub secret_key_account: String,
    /// Optional session token keychain account.
    #[serde(default)]
    pub session_token_account: Option<String>,
    /// Optional AWS profile name (for `~/.aws/credentials`).
    #[serde(default)]
    pub aws_profile: Option<String>,
    /// Bedrock model ID (e.g. `anthropic.claude-3-5-sonnet-20241022-v2:0`).
    #[serde(default)]
    pub model_id: Option<String>,
}

impl BedrockConfig {
    /// Produce the env vars needed to use Claude Code with AWS Bedrock.
    /// Reads credentials from the OS Keychain.
    pub fn env_vars(&self) -> Result<EnvMap> {
        let mut map = EnvMap::new();
        map.insert("AWS_DEFAULT_REGION".to_string(), self.region.clone());

        if !self.access_key_account.is_empty() {
            let key = retrieve_from_keychain(&self.access_key_account)?;
            map.insert("AWS_ACCESS_KEY_ID".to_string(), key);
        }
        if !self.secret_key_account.is_empty() {
            let secret = retrieve_from_keychain(&self.secret_key_account)?;
            map.insert("AWS_SECRET_ACCESS_KEY".to_string(), secret);
        }
        if let Some(ref token_account) = self.session_token_account {
            if !token_account.is_empty() {
                let token = retrieve_from_keychain(token_account)?;
                map.insert("AWS_SESSION_TOKEN".to_string(), token);
            }
        }
        if let Some(ref profile) = self.aws_profile {
            map.insert("AWS_PROFILE".to_string(), profile.clone());
        }
        if let Some(ref model) = self.model_id {
            map.insert("ANTHROPIC_MODEL".to_string(), model.clone());
        }
        Ok(map)
    }

    pub fn validate(&self) -> Result<()> {
        anyhow::ensure!(!self.region.is_empty(), "AWS region must be set");
        Ok(())
    }
}

fn retrieve_from_keychain(account: &str) -> anyhow::Result<String> {
    let entry = keyring::Entry::new("claude-sentinel", account)?;
    entry
        .get_password()
        .map_err(|e| anyhow::anyhow!("retrieving AWS key '{account}': {e}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bedrock_config_validate_empty_region_fails() {
        let cfg = BedrockConfig::default();
        assert!(cfg.validate().is_err());
    }

    #[test]
    fn test_bedrock_config_validate_with_region_passes() {
        let cfg = BedrockConfig {
            region: "us-east-1".into(),
            ..Default::default()
        };
        assert!(cfg.validate().is_ok());
    }

    #[test]
    fn test_bedrock_env_vars_includes_region() {
        let cfg = BedrockConfig {
            region: "us-west-2".into(),
            model_id: Some("anthropic.claude-3-5-sonnet-20241022-v2:0".into()),
            ..Default::default()
        };
        // This will fail trying to read keychain (empty accounts), so just test the region
        // by calling with no accounts set
        let vars = cfg.env_vars().unwrap();
        assert_eq!(vars["AWS_DEFAULT_REGION"], "us-west-2");
        assert_eq!(
            vars["ANTHROPIC_MODEL"],
            "anthropic.claude-3-5-sonnet-20241022-v2:0"
        );
    }
}
