use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Command {
    bin: String,
    version: String,
    installer: Installer,
}

#[derive(Deserialize, Debug)]
pub enum Installer {
    #[serde(rename = "cargo")]
    Cargo,
}
