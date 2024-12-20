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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_parsing() {
        let config = TaintConfig::try_default().expect("expected to find config during test");
        assert!(config.igore_label.is_some(), "ingore label should be set");

        let ignorelabel = config.igore_label.unwrap();
        assert_eq!(
            ignorelabel.key,
            "ignore".to_string(),
            "ignoreLabel is not exepcted value"
        );
        assert!(
            ignorelabel.value.is_some(),
            "exepcted ignorelabel value to be set"
        );
        assert_eq!(
            ignorelabel.value.unwrap(),
            "test".to_string(),
            "ignoreLabel value is not as expected"
        );

        assert_eq!(
            config.label_taints.len(),
            2,
            "unexpected label_taints length in test config"
        );

        assert_eq!(
            config.label_taints.first().unwrap().taint.key,
            "testKey".to_string()
        );

        assert!(config.label_taints.first().unwrap().taint.value.is_some());
        assert_eq!(
            config
                .label_taints
                .first()
                .unwrap()
                .taint
                .value
                .clone()
                .unwrap(),
            "testValue".to_string()
        );
        assert_eq!(
            config.label_taints.first().unwrap().selector.key,
            "maybit.de/taint-me".to_string()
        );
        assert!(config
            .label_taints
            .first()
            .unwrap()
            .selector
            .value
            .is_none());

        // parse second taint
        assert_eq!(
            config.label_taints.get(1).unwrap().taint.key,
            "second".to_string()
        );

        assert!(config.label_taints.get(1).unwrap().taint.value.is_some());
        assert_eq!(
            config
                .label_taints
                .get(1)
                .unwrap()
                .taint
                .value
                .clone()
                .unwrap(),
            "testValue".to_string()
        );
        assert_eq!(
            config.label_taints.get(1).unwrap().selector.key,
            "maybit.de/taint-me-please".to_string()
        );
        assert!(config.label_taints.get(1).unwrap().selector.value.is_some());
        assert_eq!(
            config
                .label_taints
                .get(1)
                .unwrap()
                .selector
                .value
                .clone()
                .unwrap(),
            "foo"
        );
    }
}
