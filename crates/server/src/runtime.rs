use crate::cli::{CliCommand, ExportTypesCliOptions, ServeCliOptions, parse_cli};
use config::{Config, Environment, File, FileFormat};
use serde::Deserialize;
use std::{
    net::{IpAddr, Ipv4Addr},
    path::PathBuf,
};

use thiserror::Error;

const DEFAULT_CONFIG_PATH: &str = "config.toml";
const DEFAULT_HOST: IpAddr = IpAddr::V4(Ipv4Addr::LOCALHOST);
const DEFAULT_PORT: u16 = 8765;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuntimeCommand {
    RunServer(RuntimeOptions),
    ExportTypes { output_path: Option<PathBuf> },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeOptions {
    pub host: IpAddr,
    pub port: u16,
    pub auth_token: String,
}

#[derive(Debug, Clone, Deserialize)]
struct BaseConfig {
    host: IpAddr,
    port: u16,
    auth_token: Option<String>,
}

#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error("config path is not valid UTF-8: {0}")]
    ConfigPathNotUtf8(String),
    #[error("failed to set default {field}: {source}")]
    SetDefault {
        field: &'static str,
        #[source]
        source: config::ConfigError,
    },
    #[error("failed to load runtime configuration: {0}")]
    LoadConfig(#[source] config::ConfigError),
    #[error("failed to deserialize runtime configuration: {0}")]
    DeserializeConfig(#[source] config::ConfigError),
    #[error(
        "missing required auth token; set it with --auth-token, VIBE_CODING_REMOTE_AUTH_TOKEN, or config.toml"
    )]
    MissingAuthToken,
}

pub fn parse_runtime_command() -> Result<RuntimeCommand, RuntimeError> {
    let cli = parse_cli();

    match cli.command {
        CliCommand::Serve(options) => load_runtime_options(options).map(RuntimeCommand::RunServer),
        CliCommand::ExportTypes(ExportTypesCliOptions { output }) => {
            Ok(RuntimeCommand::ExportTypes {
                output_path: output,
            })
        }
    }
}

pub fn parse_runtime_options() -> Result<RuntimeOptions, RuntimeError> {
    match parse_runtime_command()? {
        RuntimeCommand::RunServer(options) => Ok(options),
        RuntimeCommand::ExportTypes { .. } => {
            unreachable!("export-types does not produce runtime options")
        }
    }
}

fn load_runtime_options(cli: ServeCliOptions) -> Result<RuntimeOptions, RuntimeError> {
    let base_config = load_base_config(&cli)?;
    build_runtime_options(cli, base_config)
}

fn load_base_config(cli: &ServeCliOptions) -> Result<BaseConfig, RuntimeError> {
    let config_path = cli
        .config
        .clone()
        .unwrap_or_else(|| PathBuf::from(DEFAULT_CONFIG_PATH));
    let required = cli.config.is_some();
    let config_path = config_path
        .to_str()
        .ok_or_else(|| RuntimeError::ConfigPathNotUtf8(config_path.display().to_string()))?;

    Config::builder()
        .set_default("host", DEFAULT_HOST.to_string())
        .map_err(|source| RuntimeError::SetDefault {
            field: "host",
            source,
        })?
        .set_default("port", DEFAULT_PORT)
        .map_err(|source| RuntimeError::SetDefault {
            field: "port",
            source,
        })?
        .add_source(File::new(config_path, FileFormat::Toml).required(required))
        .add_source(Environment::with_prefix("VIBE_CODING_REMOTE"))
        .build()
        .map_err(RuntimeError::LoadConfig)?
        .try_deserialize()
        .map_err(RuntimeError::DeserializeConfig)
}

fn build_runtime_options(
    cli: ServeCliOptions,
    base_config: BaseConfig,
) -> Result<RuntimeOptions, RuntimeError> {
    let auth_token = cli
        .auth_token
        .or(base_config.auth_token)
        .filter(|token| !token.trim().is_empty())
        .ok_or(RuntimeError::MissingAuthToken)?;

    Ok(RuntimeOptions {
        host: cli.host.unwrap_or(base_config.host),
        port: cli.port.unwrap_or(base_config.port),
        auth_token,
    })
}
