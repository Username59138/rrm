use crate::configfile::ConfigFile;
use crate::configfile::Lists;
use crate::configfile::Variables;
use crate::input::get_yes_no;
use crate::launcharguments::LaunchConfig;
use std::env;
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

fn check_file(
    config_file: &ConfigFile,
    launch_config: &LaunchConfig,
    file: &PathBuf,
) -> Option<PathBuf> {
    let file = file.clone();
    let include_files: Vec<&str> = launch_config
        .include
        .as_deref()
        .unwrap_or_default()
        .split(',')
        .collect();
    let exclude_files: Vec<&str> = launch_config
        .exclude
        .as_deref()
        .unwrap_or_default()
        .split(',')
        .collect();
    let confirm_files_arguments: Vec<&str> = launch_config
        .confirm_files
        .as_deref()
        .unwrap_or_default()
        .split(',')
        .collect();
    let (blacklist_files, very_blacklist_files, confirm_list_files) =
        if let Some(lists) = &config_file.lists {
            let blacklist_files = lists.blacklist_files.clone();
            let very_blacklist_files = lists.very_blacklist_files.clone();
            let confirm_files = lists.confirm_files.clone();
            (blacklist_files, very_blacklist_files, confirm_files)
        } else {
            (None, None, None)
        };
    let allow_root_deletion = if let Some(variables) = &config_file.variables {
        variables.allow_root_deletion.clone()
    } else {
        None
    };

    if match confirm_list_files {
        Some(lists) => lists.contains(&file),
        _ => false,
    } || confirm_files_arguments.contains(&file.to_str().unwrap_or_default())
    {
        println!(
            "Do you want to add {} to delete list? [Y/n]",
            file.to_str().unwrap_or_default()
        );
        let Ok(user_input) = get_yes_no() else {
            return None;
        };
        if !user_input {
            return None;
        }
    }

    if let Some(blacklist_files) = &blacklist_files
        && blacklist_files.contains(&file)
        && !include_files.contains(&file.to_str().unwrap_or_default())
    {
        return None;
    }
    if let Some(very_blacklist_files) = &very_blacklist_files
        && very_blacklist_files.contains(&file)
    {
        return None;
    }
    let root_dir = env::current_dir()
        .ok()
        .and_then(|path| path.ancestors().last().map(PathBuf::from))
        .unwrap_or_else(|| PathBuf::from("/"));
    if let Some(allow_root_deletion) = allow_root_deletion
        && !allow_root_deletion
        && (file == root_dir || (file.parent() == Some(&root_dir)))
    {
        return None;
    };
    if exclude_files.contains(&file.to_str().unwrap_or_default()) {
        return None;
    }

    if launch_config.verbose {
        println!("{} will be deleted", file.to_str().unwrap_or_default());
    }
    Some(file)
}

fn prepare_files_to_delete(
    config_file: &ConfigFile,
    launch_config: &LaunchConfig,
) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let mut files_to_delete: Vec<PathBuf> = Vec::new();

    for object in launch_config.files_path.clone() {
        if let Some(file) = check_file(config_file, launch_config, &object)
            && !object.is_dir()
        {
            files_to_delete.push(file)
        } else if object.is_dir() {
            let mut save_directory = false;
            let filles_in_dir = object.read_dir()?;
            for file in filles_in_dir {
                let file = file?.path();

                if let Some(file) = check_file(config_file, launch_config, &file) {
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
