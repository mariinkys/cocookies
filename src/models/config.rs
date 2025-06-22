// SPDX-License-Identifier: GPL-3.0-only

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Config {
    pub id: Option<i32>,
    pub gotenberg_location: Option<String>,
}
