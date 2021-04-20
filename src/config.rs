use crate::error::ChgError;

const CHANGELOG_CONFIG_START: &str = "<!-- CHANGELOG-CONFIG";
const CHANGELOG_CONFIG_END: &str = "-->";

#[derive(Serialize,Deserialize,Debug,Default)]
pub struct ChangeLogConfig {
    git: GitConfig,
    keys: KeysConfig,
}

#[derive(Serialize,Deserialize,Debug,Default)]
struct GitConfig {
    tag_version_pattern: String,
}

#[derive(Serialize,Deserialize,Debug,Default)]
struct KeysConfig {
    issue_link: String,
    issue_key: String,
    pr_link: String,
    pr_key: String,
}

impl ChangeLogConfig {
    pub fn parse_embedded(text: &str) -> Result<ChangeLogConfig, ChgError> {
        let config = match text.find(CHANGELOG_CONFIG_START) {
            None => ChangeLogConfig::default(),
            Some(start) => {
                let text = text[start..].trim_start();
                match text.find(CHANGELOG_CONFIG_END) {
                    None => {
                        return Err(ChgError::ConfigReadError("missing end delimiter for embedded config".to_string()));
                    }
                    Some(end) => {
                        let text = text[0..end].trim_end();
                        toml::from_str(text)
                            .map_err(|e| ChgError::ConfigReadError(e.to_string()))?
                    }
                }
            }
        };
        Ok(config)
    }

    pub fn to_string_embedded(&self) -> Result<String, ChgError> {
        let config_text = toml::to_string(&self)
            .map_err(|_| ChgError::ConfigWriteError)?;
        Ok(format!("{}\n{}{}", CHANGELOG_CONFIG_START, config_text, CHANGELOG_CONFIG_END))
    }
}
