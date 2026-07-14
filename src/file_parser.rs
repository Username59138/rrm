use crate::config_file::ConfigFile;
use crate::input::get_yes_no;
use crate::launch_config::LaunchConfig;
use std::env;
use std::error::Error;
use std::path::{PathBuf, absolute};

fn return_launch_config_files(files: Option<&str>) -> Vec<PathBuf> {
    files
        .unwrap_or_default()
        .split(',')
        .map(|file| PathBuf::from(file))
        .collect()
}

pub fn check_file(
    config_file: &ConfigFile,
    launch_config: &LaunchConfig,
    file: &PathBuf,
) -> Result<Option<PathBuf>, Box<dyn Error>> {
    let root_dir = env::current_dir()
        .ok()
        .and_then(|path| path.ancestors().last().map(PathBuf::from))
        .unwrap_or_else(|| PathBuf::from("/"));

    let file = absolute(file.clone())?;
    let include_files = return_launch_config_files(launch_config.include.as_deref());
    let exclude_files = return_launch_config_files(launch_config.exclude.as_deref());
    let confirm_files_arguments =
        return_launch_config_files(launch_config.confirm_files.as_deref());

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
    } || confirm_files_arguments.contains(&file)
    {
        println!(
            "Do you want to add {} to delete list? [Y/n]",
            file.to_str().unwrap_or_default()
        );
        let user_input = get_yes_no()?;
        if !user_input {
            return Ok(None);
        }
    }

    if let Some(blacklist_files) = &blacklist_files
        && blacklist_files.contains(&file)
        && !include_files.contains(&file)
    {
        return Ok(None);
    }
    if let Some(very_blacklist_files) = &very_blacklist_files
        && very_blacklist_files.contains(&file)
    {
        return Ok(None);
    }
    if let Some(allow_root_deletion) = allow_root_deletion
        && !allow_root_deletion
        && (file == root_dir || file.parent() == Some(&root_dir))
    {
        return Ok(None);
    } else if file == root_dir || file.parent() == Some(&root_dir) {
        return Ok(None);
    };
    if exclude_files.contains(&file) {
        return Ok(None);
    }

    if launch_config.verbose {
        println!("{} will be deleted", file.to_str().unwrap_or_default());
    }
    Ok(Some(file))
}
