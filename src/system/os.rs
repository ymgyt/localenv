use serde::{de, Deserialize, Deserializer};

use std::{fmt, str::FromStr};

/// supported OS.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Os {
    Mac,
    Windows,
    Linux,
}

impl FromStr for Os {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "mac" | "macos" | "osx" => Ok(Os::Mac),
            "windows" => Ok(Os::Windows),
            "linux" => Ok(Os::Linux),
            _ => Err(format!("unexpected os: {}", s)),
        }
    }
}

impl fmt::Display for Os {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Os::Mac => write!(f, "mac"),
            Os::Windows => write!(f, "windows"),
            Os::Linux => write!(f, "linux"),
        }
    }
}

impl Os {
    fn variants() -> &'static [&'static str] {
        &["mac", "windows", "linux"]
    }

    #[cfg(target_os = "macos")]
    pub(super) fn detect() -> Os {
        Os::Mac
    }

    #[cfg(target_os = "windows")]
    fn detect() -> Os {
        Os::Windows
    }

    #[cfg(target_os = "linux")]
    fn detect() -> Os {
        Os::Linux
    }
}

struct OsVisitor;

impl<'de> de::Visitor<'de> for OsVisitor {
    type Value = Os;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Os::from_str(v).map_err(|_| de::Error::unknown_variant(v, Os::variants()))
    }
}

impl<'de> Deserialize<'de> for Os {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(OsVisitor)
    }
}
