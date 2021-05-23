use std::{
    fs::{read_to_string, write},
    path::PathBuf,
};

use util::LinkRequest;

#[derive(Debug)]
pub enum SaveError {
    AlreadyExists,
    OpenFailure,
    WriteFailure,
    Other,
}

impl SaveError {
    pub fn to_string(&self) -> &'static str {
        match self {
            SaveError::AlreadyExists => "That ID is already taken",
            SaveError::OpenFailure | SaveError::Other => "An internal error occurred.",
            SaveError::WriteFailure => "Failed to save your link on our end.",
        }
    }
}

fn already_exists(link: &String, file: &String) -> Result<(bool, String), ()> {
    let mut has_link: bool = false;
    let mut ln = String::new();
    for x in file.lines() {
        if let Some(idx) = x.find('.') {
            let (id, misc) = x.split_at(idx);
            if String::from(id) == link.to_owned() {
                has_link = true;
                ln = String::from(x);
                break;
            } else {
                continue;
            };
        } else {
            continue;
        }
    }

    Ok((has_link, ln))
}

pub const STORAGE_LOCATION: &'static str = env!("STORAGE");

pub fn store_link(link: &LinkRequest) -> Result<(), SaveError> {
    match read_to_string(STORAGE_LOCATION) {
        Ok(file) => {
            if let Ok((does_exist, _)) = already_exists(&link.id, &file) {
                if does_exist == true {
                    Err(SaveError::AlreadyExists)
                } else {
                    let formatted = format!(
                        "{}\n{}.{}.{}",
                        file,
                        link.id,
                        link.url.replace(".", "!"),
                        link.passcode
                    );
                    if let Ok(()) = write(STORAGE_LOCATION, formatted) {
                        Ok(())
                    } else {
                        Err(SaveError::WriteFailure)
                    }
                }
            } else {
                Err(SaveError::OpenFailure)
            }
        }
        Err(_) => Err(SaveError::OpenFailure),
    }
}

pub enum GetError {
    NotFound,
}

pub fn get_link(id: &String) -> Result<(String, String), GetError> {
    match read_to_string(STORAGE_LOCATION) {
        Ok(file) => {
            if let Ok((does_exist, line)) = already_exists(&id, &file) {
                if does_exist == true {
                    let (id, misc) = line.split_at(line.find('.').unwrap());
                    let misc = misc.strip_prefix('.').unwrap();

                    let (url, code) = misc.split_at(misc.find('.').unwrap());
                    let url = url.replace("!", ".");

                    match url.starts_with("http") {
                        true => Ok((url, code.to_string())),
                        false => Ok((format!("https://{}", url), code.to_string())),
                    }
                } else {
                    Err(GetError::NotFound)
                }
            } else {
                Err(GetError::NotFound)
            }
        }
        Err(error) => Err(GetError::NotFound),
    }
}

pub enum DeleteError {
    NotFound,
    File,
}

pub fn delete_link(id: &String) -> Result<(), DeleteError> {
    match read_to_string(STORAGE_LOCATION) {
        Ok(file) => {
            let mut line = String::new();
            for (idx, x) in file.lines().enumerate() {
                if let Some(idx) = x.find('.') {
                    if x.split_at(idx).0 == id {
                        line = x.to_string();
                        break;
                    } else {
                        continue;
                    }
                } else {
                    continue;
                }
            }

            file.replace(format!("{}\n", line).as_str(), "");
            Ok(())
        }
        Err(error) => Err(DeleteError::File),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn insert_link() {
        store_link(&LinkRequest {
            passcode: "test".to_string(),
            id: "test".to_string(),
            url: "https://github.com/hvlck".to_string(),
        })
        .unwrap();
    }
}
