use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Serialize, Deserialize, Clone)]
pub struct BridgeStats {
    pub(crate) skipped_messages: i32,
    pub(crate) routed_messages: i32,
    pub(crate) errors: i32,
    pub(crate) connection_error: i32,
    start_time: SystemTime,
}

impl Default for BridgeStats {
    fn default() -> Self {
        BridgeStats {
            skipped_messages: 0,
            routed_messages: 0,
            errors: 0,
            connection_error: 0,
            start_time: SystemTime::now(),
        }
    }
}

impl BridgeStats {
    pub(crate) fn as_json(&self) -> String {
        return serde_json::to_string(&self).unwrap();
    }

    fn from_json(s: &str) -> Self {
        return serde_json::from_str(s).unwrap();
    }
}
