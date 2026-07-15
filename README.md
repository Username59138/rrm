# rrm - Rust Remove 
`rrm` is advanced remove utility written on rust.
`rrm` has config, advanced confirm logic, blacklists, and many flags.


## How to install:
### Cloning and change directory to repository
```bash
git clone https://github.com/Username59138/rrm.git && cd rrm
```


### **Installing** with cargo 
```bash
cargo install --path .
```

## Config 
`rrm` has `rrm/config.toml` file which is configuration which you can change.
# rrm/config.toml example:

```toml
[variables]
allow_root_deletion = false
confirm_deleting = true
[lists]
blacklist_files = ["/home/user/important_project"]
very_blacklist_files = ["/home/user/very_important_project"]
confirm_list_files = ["/home/user/project"]
```


### Project currently in alpha
### No vibecode/ai code was used
