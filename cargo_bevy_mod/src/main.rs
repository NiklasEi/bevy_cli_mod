use bevy_utils_mod::icon::create_icons;
use bevy_utils_mod::{BevyProjectConfig, Platform};
use clap::{Args, Parser};
use std::collections::HashSet;

fn main() {
    let args = Command::parse();
    args.exec();
}

#[derive(Debug, Parser)]
#[command(bin_name = "cargo")]
pub enum Command {
    BevyMod(BevyArgs),
}

impl Command {
    pub fn exec(self) {
        match self {
            Self::BevyMod(add) => add.exec(),
        }
    }
}

#[test]
fn verify_app() {
    use clap::CommandFactory;
    Command::command().debug_assert()
}

#[derive(Debug, Args)]
#[command(version)]
pub struct BevyArgs {
    /// Path to the build data directory
    #[arg(long)]
    build_data_directory: String,
    /// Platforms to generate icons for
    #[arg(long, value_enum, num_args = 1.., value_delimiter = ' ')]
    platforms: Option<Vec<Platform>>,
}

impl BevyArgs {
    fn exec(&self) {
        create_icons(BevyProjectConfig {
            build_data_directory: self.build_data_directory.clone().into(),
            platforms: HashSet::from_iter(
                self.platforms.clone().unwrap_or(Platform::all()).drain(..),
            ),
        })
        .unwrap_or_else(|error| panic!("Failed to create icons: {error}"))
    }
}
