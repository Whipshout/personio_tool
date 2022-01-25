use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Params {
    pub delay_between_requests_ms: u64,
    pub timeout_request_seconds: u64,
}
