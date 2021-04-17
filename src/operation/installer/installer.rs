use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Installer {
    #[serde(rename = "cargo")]
    Cargo,
}
