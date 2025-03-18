use std::path::Path;
use std::io::{Error, ErrorKind};
use crate::config::write_default_config;


fn dir_exists(path: &Path) -> Result<bool, Error> {
    let exists = std::fs::exists(path);

    if exists.is_err() {
        return Err(exists.err().unwrap());
    }

    Ok(exists.ok().unwrap())
}

fn file_exists(path: &Path) -> Result<bool, Error> {
    path.try_exists()
}

fn check_filesystem_integrity() -> Result<bool, Error> {

    let mut did_changes = false;

    let dirs: Vec<&Path> = vec![
        Path::new("./user/"),
        Path::new("./res/"),
    ];

    println!("[info] checking directory...");
    for dir in dirs {
        let res = dir_exists(dir);

        if res.is_err() {
            return Err(Error::from(ErrorKind::PermissionDenied));
        }

        if !res.ok().unwrap() {
            println!("[info] created dir {}", dir.display());
            std::fs::create_dir(dir)?;
            did_changes = true;
        }
    }

    let config_path = Path::new("./user/config.toml");

    println!("[info] checking config...");
    if !file_exists(config_path)? {
        let res = write_default_config(config_path);

        if res.is_some() {
            #[allow(clippy::unnecessary_unwrap)]
            return Err(res.unwrap());
        }

        println!("[info] generated {}", config_path.display());
        did_changes = true;

    }


    Ok(did_changes)
}

pub fn init() {
    println!("[info] Initializing...");

    // init config file - doesn't check integrity tho
    println!("[info] Filesystem integrity");
    let res = check_filesystem_integrity();

    if res.is_err() {
        println!("[fatal] {}", res.err().unwrap());
        std::process::exit(1);
    }

    if !res.unwrap() {
        println!("[info] No changes were made");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dir_doesnt_exist() {
        let path = Path::new("./nonexistantdirectory/");
        let result = dir_exists(path);

        assert!(result.is_ok());
        assert!(!result.ok().unwrap());
    }

    #[test]
    fn config_file_doesnt_exist() {
        let path = Path::new("./user/missingconfig.toml");

        let result = file_exists(path);

        assert!(result.is_ok());
        assert!(!result.ok().unwrap());
    }

    #[test]
    fn filesystem_empty() {
        let res = check_filesystem_integrity();

        assert!(res.is_ok());
        assert!(res.ok().unwrap());
    }
}