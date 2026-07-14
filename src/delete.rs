use crate::config_file::ConfigFile;
use crate::config_file::Variables;
use crate::file_parser::check_file;
use crate::input::get_yes_no;
use crate::launch_config::LaunchConfig;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

fn delete_any(paths: &[PathBuf], verbose: bool) -> Result<(), Box<dyn Error>> {
    for path in paths {
        let metadata = path.metadata()?;
        if metadata.is_dir() {
            fs::remove_dir(path)?;
        } else {
            fs::remove_file(path)?;
        }
        if verbose {
            println!("Removed {}", path.to_str().unwrap_or_default());
        }
    }
    Ok(())
}

fn prepare_files_to_delete(
    config_file: &ConfigFile,
    launch_config: &LaunchConfig,
) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let mut files_to_delete: Vec<PathBuf> = Vec::new();

    for object in launch_config.files_path.clone() {
        if let Some(file) = check_file(config_file, launch_config, &object)?
            && !object.is_dir()
        {
            files_to_delete.push(file)
        } else if object.is_dir() {
            let mut save_directory = false;
            let filles_in_dir = object.read_dir()?;
            for file in filles_in_dir {
                let file = file?.path();

                if let Some(file) = check_file(config_file, launch_config, &file)? {
                    files_to_delete.push(file);
                } else {
                    save_directory = true;
                }
            }
            if !save_directory {
                files_to_delete.push(object);
            }
        }
    }
    Ok(files_to_delete)
}

pub fn start_deletion(
    config_file: &ConfigFile,
    launch_config: &LaunchConfig,
) -> Result<(), Box<dyn Error>> {
    let files_to_delete = prepare_files_to_delete(config_file, launch_config)?;
    if matches!(
        config_file.variables,
        Some(Variables {
            confirm_deleting: Some(true),
            ..
        })
    ) && !launch_config.no_confirm
        || launch_config.confirm
    {
        println!(
            "{} files will be deleted. Continue? [Y/n]",
            files_to_delete.len()
        );
        let user_input = get_yes_no()?;
        if !user_input {
            return Ok(());
        }
    }
    delete_any(&files_to_delete, launch_config.verbose)?;
    Ok(())
}
