use bevy_utils_mod::icon::create_icons;
use bevy_utils_mod::{BevyProjectConfig, BuildTargets};
use clap::{Args, Parser};

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
}

impl BevyArgs {
    fn exec(&self) {
        create_icons(BevyProjectConfig {
            build_data_directory: self.build_data_directory.clone().into(),
            targets: BuildTargets::all(),
        })
        .expect("Failed to create icons")
    }
}
