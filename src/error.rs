#![allow(missing_docs)]

use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("IO error: `{0}`")]
    Io(#[from] std::io::Error),
    #[error("D-Bus error: `{0}`")]
    Dbus(#[from] dbus::Error),
    #[error("D-Bus string error: `{0}`")]
    DbusString(String),
    #[error("D-Bus argument error: `{0}`")]
    DbusArgument(String),
    #[error("Receiver error: `{0}`")]
    Receiver(#[from] std::sync::mpsc::RecvError),
    #[error("TOML parsing error: `{0}`")]
    Toml(#[from] toml::de::Error),
    #[error("Scan error: `{0}`")]
    Scanf(String),
    #[error("Integer conversion error: `{0}`")]
    IntegerConversion(#[from] std::num::TryFromIntError),
    #[error("Template error: `{0}`")]
    Template(#[from] tera::Error),
    #[error("Template parse error:\n{0}")]
    TemplateParse(String),
    #[error("Template render error:\n{0}")]
    TemplateRender(String),
    #[error("System time error: `{0}`")]
    SystemTime(#[from] std::time::SystemTimeError),
    #[error("Config error: `{0}`")]
    Config(String),
}

/// Type alias for the standard [`Result`] type.
pub type Result<T> = std::result::Result<T, Error>;
