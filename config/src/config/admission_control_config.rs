// Copyright (c) The Libra Core Contributors
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(default)]
pub struct AdmissionControlConfig {
    pub address: String,
    pub admission_control_service_port: u16,
    pub need_to_check_mempool_before_validation: bool,
    pub max_concurrent_inbound_syncs: usize,
    pub upstream_proxy_timeout: Duration,
}

impl Default for AdmissionControlConfig {
    fn default() -> AdmissionControlConfig {
        AdmissionControlConfig {
            address: "0.0.0.0".to_string(),
            admission_control_service_port: 8001,
            need_to_check_mempool_before_validation: false,
            max_concurrent_inbound_syncs: 100,
            upstream_proxy_timeout: Duration::from_secs(1),
        }
    }
}
