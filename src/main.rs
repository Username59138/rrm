use rrm::config_file::ConfigFile;
use rrm::delete;
use rrm::launch_config::LaunchConfig;

fn main() {
    let config = ConfigFile::new().unwrap();
    let arguments = LaunchConfig::new().unwrap();
    delete::start_deletion(&config, &arguments).unwrap();
}
