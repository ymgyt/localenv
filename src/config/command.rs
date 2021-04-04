use serde::Deserialize;

use crate::operation::installer::Installer;

#[derive(Deserialize, Debug, Clone)]
pub struct Command {
    pub bin: String,
    pub version: String,
    pub installer: Installer,
}

