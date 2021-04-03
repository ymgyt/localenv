use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Command {
    pub bin: String,
    pub version: String,
    pub installer: Installer,
}

#[derive(Deserialize, Debug, Clone)]
pub enum Installer {
    #[serde(rename = "cargo")]
    Cargo,
}
