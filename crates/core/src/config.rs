use serde::Serialize;

#[derive(Debug, Clone Serialize, Deserialize)]
pub struct AppConfig {
    pub java_version_checks: bool,
    pub setup: bool,
    
}
