use crate::error::{Error, Result};
use crate::notification::{Notification, NotificationFilter, Urgency};
use colorsys::Rgb;
use rust_embed::RustEmbed;
use serde::de::{Deserializer, Error as SerdeError};
use serde::ser::Serializer;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::result::Result as StdResult;
use std::str::{self, FromStr};
use std::time::{SystemTime, UNIX_EPOCH};
use tera::Tera;
use tracing::Level;
use xdg::BaseDirectories;

/// Environment variable for the configuration file.
const CONFIG_ENV: &str = "NOFI_CONFIG";

/// Name of the default configuration file.
const DEFAULT_CONFIG: &str = concat!(env!("CARGO_PKG_NAME"), ".toml");

/// Embedded (default) configuration.
#[derive(Debug, RustEmbed)]
#[folder = "config/"]
struct EmbeddedConfig;

/// Configuration.
#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    /// Global configuration.
    pub global: GlobalConfig,
    /// Configuration for low urgency.
    pub urgency_low: UrgencyConfig,
    /// Configuration for normal urgency.
    pub urgency_normal: UrgencyConfig,
    /// Configuration for critical urgency.
    pub urgency_critical: UrgencyConfig,
}

impl Config {
    /// Parses the configuration file.
    pub fn parse(xdg_dirs: &BaseDirectories) -> Result<Self> {
        for config_path in [
            env::var(CONFIG_ENV).ok().map(PathBuf::from),
            Some(xdg_dirs.place_config_file(DEFAULT_CONFIG)?),
        ]
        .iter()
        .flatten()
        {
            if config_path.exists() {
                let contents = fs::read_to_string(config_path)?;
                let config = toml::from_str(&contents)?;
                return Ok(config);
            }
        }
        if let Some(embedded_config) = EmbeddedConfig::get(DEFAULT_CONFIG)
            .and_then(|v| String::from_utf8(v.data.as_ref().to_vec()).ok())
        {
            let config = toml::from_str(&embedded_config)?;
            Ok(config)
        } else {
            Err(Error::Config(String::from("configuration file not found")))
        }
    }

    /// Returns the appropriate urgency configuration.
    pub fn get_urgency_config(&self, urgency: &Urgency) -> UrgencyConfig {
        match urgency {
            Urgency::Low => self.urgency_low.clone(),
            Urgency::Normal => self.urgency_normal.clone(),
            Urgency::Critical => self.urgency_critical.clone(),
        }
    }
}

/// Global configuration.
#[derive(Debug, Deserialize, Serialize)]
pub struct GlobalConfig {
    /// Log verbosity.
    #[serde(deserialize_with = "deserialize_level_from_string", skip_serializing)]
    pub log_verbosity: Level,
    /// Template for the notification message.
    pub template: String,
}

/// Custom deserializer implementation for converting `String` to [`Level`]
fn deserialize_level_from_string<'de, D>(deserializer: D) -> StdResult<Level, D::Error>
where
    D: Deserializer<'de>,
{
    let value: String = Deserialize::deserialize(deserializer)?;
    Level::from_str(&value).map_err(SerdeError::custom)
}

/// Urgency configuration.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UrgencyConfig {
    /// Background color.
    #[serde(
        deserialize_with = "deserialize_rgb_from_string",
        serialize_with = "serialize_rgb_to_string"
    )]
    pub background: Rgb,
    /// Foreground color.
    #[serde(
        deserialize_with = "deserialize_rgb_from_string",
        serialize_with = "serialize_rgb_to_string"
    )]
    pub foreground: Rgb,
    /// Timeout value.
    pub timeout: u32,
    /// Whether if auto timeout is enabled.
    pub auto_clear: Option<bool>,
    /// Text.
    pub text: Option<String>,
    /// Custom OS commands to run.
    pub custom_commands: Option<Vec<CustomCommand>>,
}

/// Custom deserializer implementation for converting `String` to [`Rgb`]
fn deserialize_rgb_from_string<'de, D>(deserializer: D) -> StdResult<Rgb, D::Error>
where
    D: Deserializer<'de>,
{
    let value: String = Deserialize::deserialize(deserializer)?;
    Rgb::from_hex_str(&value).map_err(SerdeError::custom)
}

/// Custom serializer implementation for converting [`Rgb`] to `String`
fn serialize_rgb_to_string<S>(value: &Rgb, s: S) -> StdResult<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(&value.to_hex_string())
}

impl UrgencyConfig {
    /// Runs the custom OS commands that are determined by configuration.
    pub fn run_commands(&self, notification: &Notification) -> Result<()> {
        if let Some(commands) = &self.custom_commands {
            for command in commands {
                if let Some(filter) = &command.filter {
                    if !notification.matches_filter(filter) {
                        continue;
                    }
                }
                if (notification.timestamp
                    + notification.expire_timeout.unwrap_or_default().as_secs())
                    < SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs()
                {
                    continue;
                }
                tracing::trace!("running command: {:#?}", command);
                let command = Tera::one_off(
                    &command.command,
                    &notification.into_context(
                        self.text
                            .clone()
                            .unwrap_or_else(|| notification.urgency.to_string()),
                        0,
                    )?,
                    true,
                )?;
                Command::new("sh").args(["-c", &command]).spawn()?;
            }
        }
        Ok(())
    }
}

/// Custom OS commands along with notification filters.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CustomCommand {
    /// Notification message filter.
    #[serde(deserialize_with = "deserialize_filter_from_string", default)]
    filter: Option<NotificationFilter>,
    /// Command.
    command: String,
}

/// Custom deserializer implementation for converting `String` to [`NotificationFilter`]
fn deserialize_filter_from_string<'de, D>(
    deserializer: D,
) -> StdResult<Option<NotificationFilter>, D::Error>
where
    D: Deserializer<'de>,
{
    let value: Option<String> = Deserialize::deserialize(deserializer)?;
    match value {
        Some(v) => serde_json::from_str(&v).map_err(SerdeError::custom),
        None => Ok(None),
    }
}
