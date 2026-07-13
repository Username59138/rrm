use crate::config_file::ConfigFile;
use crate::input::get_yes_no;
use crate::launch_config::LaunchConfig;
use std::env;
use std::path::PathBuf;

pub fn check_file(
    config_file: &ConfigFile,
    launch_config: &LaunchConfig,
    file: &PathBuf,
) -> Option<PathBuf> {
    let mut file = file.clone();
    if !file.starts_with("/") {
        file.push(env::current_dir().unwrap_or_else(|_| PathBuf::from("/")));
    }
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
        && (file == root_dir || file.parent() == Some(&root_dir))
    {
        return None;
    } else if file == root_dir || file.parent() == Some(&root_dir) {
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
