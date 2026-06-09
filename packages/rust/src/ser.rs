//! Pydantic 风格的序列化便捷方法。

#[cfg(feature = "json")]
pub trait ToJson: serde::Serialize {
    fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
    fn to_json_pretty(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}

#[cfg(feature = "json")]
impl<T: serde::Serialize> ToJson for T {}

#[cfg(feature = "json")]
pub trait FromJson: serde::de::DeserializeOwned {
    fn from_json(s: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(s)
    }
}

#[cfg(feature = "json")]
impl<T: serde::de::DeserializeOwned> FromJson for T {}

#[cfg(feature = "yaml")]
pub trait ToYaml: serde::Serialize {
    fn to_yaml(&self) -> Result<String, serde_yaml::Error> {
        serde_yaml::to_string(self)
    }
}

#[cfg(feature = "yaml")]
impl<T: serde::Serialize> ToYaml for T {}

#[cfg(feature = "yaml")]
pub trait FromYaml: serde::de::DeserializeOwned {
    fn from_yaml(s: &str) -> Result<Self, serde_yaml::Error> {
        serde_yaml::from_str(s)
    }
}

#[cfg(feature = "yaml")]
impl<T: serde::de::DeserializeOwned> FromYaml for T {}
