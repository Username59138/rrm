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

fn prepare_objects_to_delete(
    objects: Vec<PathBuf>,
    config_file: &ConfigFile,
    launch_config: &LaunchConfig,
) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let mut files_to_delete: Vec<PathBuf> = Vec::new();

    for object in objects {
        if !object.is_dir() {
            if check_file(config_file, launch_config, &object)? {
                files_to_delete.push(object)
            }
        } else {
            let mut save_directory = false;
            let filles_in_dir = object.read_dir()?;
            for file in filles_in_dir {
                let file = file?.path();

                if check_file(config_file, launch_config, &file)? {
                    let checked_object =
                        prepare_objects_to_delete(vec![file.clone()], config_file, launch_config)?;
                    if !checked_object.contains(&file) {
                        save_directory = true;
                    }
                    files_to_delete.extend_from_slice(&checked_object);
                } else {
                    save_directory = true;
                }
            }
            if !save_directory && check_file(config_file, launch_config, &object)? {
                files_to_delete.push(object);
            }
        }
    }
    Ok(files_to_delete)
}

fn prepare_files_to_delete(
    config_file: &ConfigFile,
    launch_config: &LaunchConfig,
) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let mut files_to_delete: Vec<PathBuf> = Vec::new();

    let prepared_files =
        prepare_objects_to_delete(launch_config.files_path.clone(), config_file, launch_config)?;
    files_to_delete.extend_from_slice(&prepared_files);
    if launch_config.verbose {
        files_to_delete.iter().for_each(|file| {
            println!(
                "File: {} will be deleted",
                file.to_str().unwrap_or_default()
            );
        });
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
