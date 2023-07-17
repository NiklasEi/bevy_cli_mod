use std::collections::HashSet;
use std::path::PathBuf;

pub mod icon;

pub struct BevyProjectConfig {
    pub build_data_directory: PathBuf,
    pub platforms: HashSet<Platform>,
}

#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub enum Platform {
    Windows,
    Linux,
    Mac,
    Web,
    Android,
    Ios,
}

impl Platform {
    pub fn all() -> Vec<Platform> {
        let mut platforms = vec![];

        platforms.push(Platform::Windows);
        platforms.push(Platform::Linux);
        platforms.push(Platform::Mac);
        platforms.push(Platform::Web);
        platforms.push(Platform::Android);
        platforms.push(Platform::Ios);

        platforms
    }
}
