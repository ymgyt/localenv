use std::path::PathBuf;

use tokio::process;

use crate::{prelude::*, system};

const CARGO_BIN: &str = "cargo";

/// cargo package.
#[derive(Debug, PartialEq, Clone)]
pub struct Package {
    name: String,
    bin: String,
    version: semver::Version,
    local_path: Option<PathBuf>,
}

impl Package {
    pub fn bin(&self) -> &str {
        self.bin.as_str()
    }

    fn with_bin<T: Into<String>>(mut self, bin: T) -> Self {
        self.bin = bin.into();
        self
    }
}

pub struct Cargo<Cmd> {
    cmd: Cmd,
}

impl Cargo<process::Command> {
    pub fn new() -> Result<Self> {
        let cargo_path = system::resolve_binary_path(CARGO_BIN)?;
        let cmd = process::Command::new(&cargo_path);

        Ok(Self { cmd })
    }

    pub async fn list_installed_packages(&mut self) -> Result<Vec<Package>> {
        let output = self
            .cmd
            .args(&["install", "--list"])
            .output()
            .await
            .expect("cargo install --list failed");

        let output = String::from_utf8_lossy(output.stdout.as_slice());

        parse_install_list(output)
    }
}

fn parse_install_list(s: impl AsRef<str>) -> Result<Vec<Package>> {
    match parser::package_list(s.as_ref()) {
        Ok((_, packages)) => Ok(packages),
        Err(nom_err) => Err(Error::from(ErrorKind::Internal(format!(
            "failed to parse installed package list: {:?}",
            nom_err
        )))),
    }
}

/// cargo command output parser moduel.
mod parser {
    use super::Package;
    use nom::bytes::complete;
    use nom::character;
    use nom::combinator;
    use nom::multi;
    use nom::sequence;
    use nom::IResult;
    use std::path::PathBuf;

    /// parse cargo package name.
    fn package_name(i: &str) -> IResult<&str, &str> {
        complete::take_while(|c: char| c.is_alphanumeric() || c == '-' || c == '_')(i)
    }

    /// parse cargo package semantic version.
    fn version(i: &str) -> IResult<&str, semver::Version> {
        combinator::map_res(
            sequence::preceded(
                complete::tag("v"),
                complete::take_while(|c| ('0'..='9').contains(&c) || c == '.'),
            ),
            semver::Version::parse,
        )(i)
    }

    /// consume separator spaces.
    fn space(i: &str) -> IResult<&str, ()> {
        complete::take_while(|c: char| c.is_whitespace())(i).map(|(remain, _)| (remain, ()))
    }

    /// parse local path enclosed in parentheses.
    fn local_path(i: &str) -> IResult<&str, PathBuf> {
        combinator::map(
            sequence::delimited(
                complete::tag("("),
                complete::take_until(")"),
                complete::tag(")"),
            ),
            |path| PathBuf::from(path),
        )(i)
    }

    /// parse package binary line.
    fn bin_line(i: &str) -> IResult<&str, String> {
        combinator::map(
            sequence::tuple((
                complete::take_while1(|c: char| c.is_whitespace()),
                package_name, // define bin_name if needed.
            )),
            |(_, bin)| String::from(bin),
        )(i)
    }

    /// parse package binary lines.
    fn bin_lines(i: &str) -> IResult<&str, Vec<String>> {
        multi::separated_list1(character::complete::line_ending, bin_line)(i)
    }

    /// parse first line in packages list entry.
    fn package_line(i: &str) -> IResult<&str, Package> {
        combinator::map(
            sequence::terminated(
                sequence::tuple((
                    package_name,
                    space,
                    version,
                    combinator::opt(sequence::preceded(space, local_path)),
                )),
                complete::tag(":"),
            ),
            |(name, _, v, local_path)| Package {
                name: name.to_owned(),
                version: v,
                local_path,
                bin: "".to_owned(),
            },
        )(i)
    }

    /// parse package entry.
    fn package_entry(i: &str) -> IResult<&str, Package> {
        combinator::map(sequence::tuple((package_line, bin_lines)), |(pkg, bins)| {
            pkg.with_bin(bins.first().expect("at least one binary"))
        })(i)
    }

    /// parse cargo install --list output.
    pub(super) fn package_list(i: &str) -> IResult<&str, Vec<Package>> {
        multi::many1(sequence::preceded(
            combinator::opt(character::complete::newline),
            package_entry,
        ))(i)
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use nom::Err as NomErr;
        use pretty_assertions::assert_eq;
        use std::ffi::OsStr;

        impl Package {
            fn with_local_path<T: ?Sized + AsRef<OsStr>>(mut self, path: &T) -> Self {
                self.local_path = Some(PathBuf::from(path));
                self
            }
        }

        #[test]
        fn test_package_name() {
            assert_eq!(package_name("alacritty"), Ok(("", "alacritty")));
        }
        #[test]
        fn test_version() {
            assert_eq!(version("v0.1.2"), Ok(("", semver::Version::new(0, 1, 2))));
            assert_eq!(
                version("xxx"),
                Err(NomErr::Error(nom::error::Error::new(
                    "xxx",
                    nom::error::ErrorKind::Tag
                )))
            );
        }
        #[test]
        fn test_local_path() {
            assert_eq!(
                local_path("(/Users/ymgyt/hello/rust)"),
                Ok(("", PathBuf::from("/Users/ymgyt/hello/rust")))
            );
            assert_eq!(
                local_path("(/Users/ymgyt/hello/rust"),
                Err(NomErr::Error(nom::error::Error::new(
                    "/Users/ymgyt/hello/rust",
                    nom::error::ErrorKind::TakeUntil
                )))
            );
        }
        #[test]
        fn test_bin_line() {
            assert_eq!(bin_line("    nu"), Ok(("", "nu".to_owned())));
            assert_eq!(
                bin_line("nu"),
                Err(NomErr::Error(nom::error::Error::new(
                    "nu",
                    nom::error::ErrorKind::TakeWhile1
                )))
            );
        }
        #[test]
        fn test_bin_lines() {
            assert_eq!(
                bin_lines("    nu_1\n    nu_2\nripgrep"),
                Ok(("\nripgrep", vec!["nu_1".into(), "nu_2".into()]))
            );
        }
        #[test]
        fn test_package_line() {
            assert_eq!(package_line("bat v0.17.1:"), Ok(("", pkg_bat())));
            assert_eq!(
                package_line("bat v0.17.1 (/Users/ymgyt/hello/rust):"),
                Ok(("", pkg_bat().with_local_path("/Users/ymgyt/hello/rust"))),
            );
        }
        #[test]
        fn test_package_entry() {
            assert_eq!(
                package_entry("bat v0.17.1:\n    bat"),
                Ok(("", pkg_bat().with_bin("bat")))
            );
            assert_eq!(
                package_entry("bat v0.17.1:\n    bat_1\n    bat_2\nripgrep"),
                Ok(("\nripgrep", pkg_bat().with_bin("bat_1")))
            );
        }
        #[test]
        fn parse_install_list() {
            let s = r"alacritty v0.7.2 (/Users/ymgyt/rs/alacritty/alacritty):
    alacritty
bat v0.17.1:
    bat
cargo-make v0.32.12:
    cargo-make
    makers
nu v0.29.1 (/Users/ymgyt/rs/nushell):
    nu
    nu_plugin_core_fetch
    nu_plugin_core_inc
    nu_plugin_core_match
    nu_plugin_core_post
    nu_plugin_core_ps
    nu_plugin_core_sys
    nu_plugin_core_textview
    nu_plugin_extra_binaryview
    nu_plugin_extra_chart_bar
    nu_plugin_extra_chart_line
    nu_plugin_extra_from_bson
    nu_plugin_extra_from_sqlite
    nu_plugin_extra_s3
    nu_plugin_extra_selector
    nu_plugin_extra_start
    nu_plugin_extra_to_bson
    nu_plugin_extra_to_sqlite
    nu_plugin_extra_tree
    nu_plugin_extra_xpath
ripgrep v12.1.1:
    rg
";
            let want = vec![
                Package {
                    name: "alacritty".to_owned(),
                    bin: "alacritty".to_owned(),
                    version: semver::Version::new(0, 7, 2),
                    local_path: Some(PathBuf::from("/Users/ymgyt/rs/alacritty/alacritty")),
                },
                Package {
                    name: "bat".to_owned(),
                    bin: "bat".to_owned(),
                    version: semver::Version::new(0, 17, 1),
                    local_path: None,
                },
                Package {
                    name: "cargo-make".to_owned(),
                    bin: "cargo-make".to_owned(),
                    version: semver::Version::new(0, 32, 12),
                    local_path: None,
                },
                Package {
                    name: "nu".to_owned(),
                    bin: "nu".to_owned(),
                    version: semver::Version::new(0, 29, 1),
                    local_path: Some(PathBuf::from("/Users/ymgyt/rs/nushell")),
                },
                Package {
                    name: "ripgrep".to_owned(),
                    bin: "rg".to_owned(),
                    version: semver::Version::new(12, 1, 1),
                    local_path: None,
                },
            ];

            assert_eq!(package_list(s), Ok(("\n", want)));
        }

        fn pkg_bat() -> Package {
            Package {
                name: "bat".to_owned(),
                version: semver::Version::new(0, 17, 1),
                local_path: None,
                bin: "".to_owned(),
            }
        }
    }
}
