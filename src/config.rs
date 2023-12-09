use serde::{Deserialize, Serialize};

pub static VEG_CONFIG_NAME: &str = "veg-config";

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct VegConfig {
    pub token: String,
}
