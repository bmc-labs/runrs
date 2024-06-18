// Copyright 2024 bmc::labs GmbH. All rights reserved.

use serde::Serialize;

/// These settings are global. They apply to all runners. Documentation of fields found here:
/// https://docs.gitlab.com/runner/configuration/advanced-configuration.html#the-global-section
#[derive(Debug, Serialize)]
pub struct GlobalSection {
    pub concurrent: u32,
    pub check_interval: u32,
}

impl Default for GlobalSection {
    fn default() -> Self {
        Self {
            concurrent: 4,
            check_interval: 3,
        }
    }
}
