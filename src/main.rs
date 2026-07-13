use rrm::configfile::ConfigFile;
use rrm::delete;
use rrm::launcharguments::LaunchConfig;

fn main() {
    let config = ConfigFile::new().unwrap();
    let arguments = LaunchConfig::new().unwrap();
    delete::start_deletion(&config, &arguments).unwrap();
}
