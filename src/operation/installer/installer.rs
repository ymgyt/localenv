use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub enum Installer {
    #[serde(rename = "cargo")]
    Cargo,
}

