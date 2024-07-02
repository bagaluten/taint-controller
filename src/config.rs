use config::Config;
use k8s_openapi::api::core::v1::Taint;

// Represents a label with its value.
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub struct Label {
    pub key: String,
    pub value: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub struct TaintLabel {
    pub taint: Taint,
    pub selector: Label,
}

#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub struct TaintConfig {
    #[serde(rename = "labelTaints")]
    pub label_taints: Vec<TaintLabel>,

    #[serde(rename = "ignoreLabel")]
    pub igore_label: Option<Label>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConfigurationError {
    pub message: String,
}

impl std::fmt::Display for ConfigurationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ConfigurationError: {}", self.message)
    }
}

impl TaintConfig {
    pub fn try_default() -> Result<TaintConfig, ConfigurationError> {
        let config = Config::builder()
            .add_source(config::File::with_name("config/taint-controller.yaml"))
            .build()
            .map_err(|e| ConfigurationError {
                message: format!("Failed to load configuration: {}", e),
            })?;

        let taint_config: TaintConfig =
            config.try_deserialize().map_err(|e| ConfigurationError {
                message: format!("Failed to parse configuration: {}", e),
            })?;

        Ok(taint_config)
    }
}
