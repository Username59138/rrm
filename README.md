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

## rrm/config.toml example:

```toml
[variables]
allow_root_deletion = false
confirm_deleting = true
[lists]
blacklist = ["/home/user/important_project"]
```


Project currently in beta
