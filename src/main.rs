use std::{io, sync::Arc};

use hyprland::{
    data::Monitors,
    event_listener::AsyncEventListener,
    keyword::{Keyword, OptionValue},
    shared::HyprData,
};
use thiserror::Error;

use crate::config::Config;

mod config;

const APP_NAME: &str = "thales";
const CONFIG_FILE: &str = "config.toml";

#[derive(Error, Debug)]
enum Error {
    #[error("Io error: {0}")]
    IoError(#[from] io::Error),
    #[error("Serde error: {0}")]
    TomlDeError(#[from] toml::de::Error),
    #[error("Hyprland error: {0}")]
    HyprlandError(#[from] hyprland::error::HyprError),
    #[error("Failed to find config directory")]
    ConfigDirectoryError,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), self::Error> {
    let config = Arc::new(Config::load_from_file()?);
    let mut event_listener = AsyncEventListener::new();

    set_state_async(config.clone()).await?;

    let added_config = config.clone();
    event_listener.add_monitor_added_handler(move |event| {
        let config = added_config.clone();
        Box::pin(async move {
            config
                .monitors
                .iter()
                .filter(|monitor| monitor.name == event.name)
                .filter_map(|monitor| monitor.on_added.as_ref())
                .flatten()
                .for_each(|command| {
                    run_command(command.to_string());
                });
        })
    });

    let removed_config = config.clone();
    event_listener.add_monitor_removed_handler(move |event| {
        let config = removed_config.clone();
        Box::pin(async move {
            config
                .monitors
                .iter()
                .filter(|monitor| monitor.name == event)
                .filter_map(|monitor| monitor.on_removed.as_ref())
                .flatten()
                .for_each(|command| {
                    run_command(command.to_string());
                });
            {}
        })
    });

    let reload_config = config.clone();
    event_listener.add_config_reloaded_handler(move || {
        let config = reload_config.clone();
        Box::pin(async move {
            let _ = set_state_async(config).await;
        })
    });

    event_listener.start_listener_async().await?;
    Ok(())
}

fn run_command(command: String) {
    println!("Running: {}", command);
    Keyword::set("monitor", OptionValue::String(command))
        .inspect_err(|err| println!("Error on removed: {}", err))
        .ok();
}

async fn set_state_async(config: Arc<Config>) -> hyprland::Result<()> {
    let monitors = Monitors::get_async().await?;
    for config in config.monitors.iter() {
        for monitor in monitors.iter() {
            if monitor.name != config.name {
                continue;
            }
            if monitor.disabled {
                config.on_removed.as_ref().inspect(|commands| {
                    commands
                        .iter()
                        .for_each(|command| run_command(command.to_string()));
                });
            } else {
                config.on_added.as_ref().inspect(|commands| {
                    commands
                        .iter()
                        .for_each(|command| run_command(command.to_string()))
                });
            }
        }
    }
    Ok(())
}
