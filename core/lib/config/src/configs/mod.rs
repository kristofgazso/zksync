// Public re-exports
pub use self::{
    api::ApiConfig, chain::ChainConfig, db::DBConfig, eth_client::ETHClientConfig,
    eth_sender::ETHSenderConfig, eth_watch::ETHWatchConfig, misc::MiscConfig, prover::ProverConfig,
    ticker::TickerConfig,
};

pub mod api;
pub mod chain;
pub mod contracts;
pub mod db;
pub mod eth_client;
pub mod eth_sender;
pub mod eth_watch;
pub mod misc;
pub mod prover;
pub mod ticker;

/// Convenience macro that loads the structure from the environment variable given the prefix.
///
/// # Panics
///
/// Panics if the config cannot be loaded from the environment variables.
#[macro_export]
macro_rules! envy_load {
    ($name:expr, $prefix:expr) => {
        envy::prefixed($prefix)
            .from_env()
            .unwrap_or_else(|err| panic!("Cannot load config <{}>: {}", $name, err))
    };
}

/// Convenience macro that loads the structure from the TOML file given the path.
///
/// # Panics
///
/// Panics if the config cannot be loaded from the file.
#[macro_export]
macro_rules! toml_load {
    ($name:expr, $path:expr) => {{
        let file_contents = std::fs::read_to_string($path).unwrap_or_else(|err| {
            panic!(
                "Cannot load config <{}> from file <{}>: {}",
                $name, $path, err
            )
        });
        toml::from_str(&file_contents).unwrap_or_else(|err| {
            panic!(
                "Cannot parse config <{}> from file <{}>: {}",
                $name, $path, err
            )
        })
    }};
}
