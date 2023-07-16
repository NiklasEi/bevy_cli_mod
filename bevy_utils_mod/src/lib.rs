use std::collections::HashSet;
use std::path::PathBuf;

pub mod icon;

pub struct BevyProjectConfig {
    pub build_data_directory: PathBuf,
    pub targets: HashSet<BuildTargets>,
}

#[derive(Eq, Hash, PartialEq)]
pub enum BuildTargets {
    Windows,
    Linux,
    Mac,
    Web,
    Android,
    Ios,
}

impl BuildTargets {
    pub fn all() -> HashSet<BuildTargets> {
        let mut targets: HashSet<BuildTargets> = Default::default();

        targets.insert(BuildTargets::Windows);
        targets.insert(BuildTargets::Linux);
        targets.insert(BuildTargets::Mac);
        targets.insert(BuildTargets::Web);
        targets.insert(BuildTargets::Android);
        targets.insert(BuildTargets::Ios);

        targets
    }
}
