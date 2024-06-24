// Copyright 2024 bmc::labs GmbH. All rights reserved.

use serde::Serialize;
use typed_builder::TypedBuilder;

/// These settings are global. They apply to all runners. Documentation of fields found here:
/// https://docs.gitlab.com/runner/configuration/advanced-configuration.html#the-global-section
#[derive(Debug, TypedBuilder, Serialize)]
pub struct GlobalSection {
    #[builder(default = 1)]
    pub concurrent: u32,
    #[builder(default = 3)]
    pub check_interval: u32,
}

impl Default for GlobalSection {
    fn default() -> Self {
        Self::builder().build()
    }
}
