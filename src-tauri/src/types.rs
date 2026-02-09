use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum CheckStatus {
    Pass,
    Fail,
    Warning,
    Skipped,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckResult {
    pub id: String,
    pub label: String,
    pub status: CheckStatus,
    pub message: String,
    pub remediation: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScanResult {
    pub checks: Vec<CheckResult>,
    pub timestamp: String,
    pub os_info: String,
    pub overall: CheckStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScanError {
    RegistryAccessDenied(String),
    ComInitFailed(String),
    Unexpected(String),
}

impl std::fmt::Display for ScanError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScanError::RegistryAccessDenied(msg) => write!(f, "Registry access denied: {}", msg),
            ScanError::ComInitFailed(msg) => write!(f, "COM initialization failed: {}", msg),
            ScanError::Unexpected(msg) => write!(f, "Unexpected error: {}", msg),
        }
    }
}
