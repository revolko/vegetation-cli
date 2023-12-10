use serde::{Deserialize, Serialize};

pub static VEG_CONFIG_NAME: &str = "veg-config";

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct VegConfig {
    pub token: String,
    pub url: String,
}

impl Default for VegConfig {
    fn default() -> Self {
        return Self {
            token: String::new(),
            url: String::from("https://vegetation-2yp6yw6kyq-lm.a.run.app")
        }
    }
}
