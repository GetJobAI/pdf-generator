use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    #[serde(default = "default_host")]
    pub host: String,

    #[serde(default = "default_port")]
    pub port: u16,

    #[serde(default = "default_rust_log")]
    pub rust_log: String,
}

fn default_host() -> String {
    "0.0.0.0".into()
}

const fn default_port() -> u16 {
    8080
}

fn default_rust_log() -> String {
    "info".into()
}

impl Config {
    pub fn load() -> anyhow::Result<Self> {
        envy::from_env::<Self>()
            .map_err(|e| anyhow::anyhow!("Failed to load config from environment: {e}"))
    }
}
