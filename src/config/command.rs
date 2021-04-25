use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Commands {
    pub cargo: Option<Vec<CargoCommand>>,
    pub brew: Option<Vec<BrewCommand>>,
    pub go: Option<Vec<GoCommand>>,
}

#[derive(Deserialize, Debug, Clone)]
pub enum Command {
    Cargo(CargoCommand),
    Brew(BrewCommand),
    Go(GoCommand),
}

impl Command {
    pub fn bin(&self) -> &str {
        match &self {
            Command::Cargo(cmd) => cmd.bin(),
            Command::Brew(cmd) => cmd.bin(),
            Command::Go(cmd) => cmd.bin(),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct CommandBase {
    package: String,
    bin: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CargoCommand {
    #[serde(flatten)]
    base: CommandBase,
}

#[derive(Deserialize, Debug, Clone)]
pub struct BrewCommand {
    #[serde(flatten)]
    base: CommandBase,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GoCommand {
    #[serde(flatten)]
    base: CommandBase,
}

impl CommandBase {
    pub fn bin(&self) -> &str {
        if let Some(ref bin) = self.bin {
            bin
        } else {
            &self.package
        }
    }

    pub fn package(&self) -> &str {
        &self.package
    }
}

macro_rules! delegate_base {
    ($c:ty) => {
        impl $c {
            pub fn bin(&self) -> &str {
                self.base.bin()
            }
            pub fn package(&self) -> &str {
                self.base.package()
            }
        }
    };
}

delegate_base!(CargoCommand);
delegate_base!(BrewCommand);
delegate_base!(GoCommand);
