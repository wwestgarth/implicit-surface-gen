use clap::Parser;
use config::{Config, ConfigError, File};
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct View {
    pub height: u64,
    pub width: u64,
}

#[derive(Debug, Deserialize)]
pub struct Sampling {
    pub max_depth: u64,
    pub samples_per_pixel: u64,
}

#[derive(Debug, Deserialize)]

pub struct Settings {
    pub output: String,
    pub view: View,
    pub sampling: Sampling,
}

impl Settings {
    pub fn new(p: &str) -> Result<Self, ConfigError> {
        let s = Config::builder().add_source(File::with_name(p)).build()?;

        s.try_deserialize()
    }
}

/// Ray tracer to view implicit surfaces
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// path to a config file
    #[arg(short, long, default_value = "config.toml")]
    pub config: String,
}
