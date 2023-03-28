use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub(crate) struct Config {}

impl Config {
    pub fn new() -> Self {
        Self {}
    }
}
