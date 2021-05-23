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

fn already_exists(link: &String, file: String) -> Result<bool, ()> {
    let mut has_link: bool = false;
    for x in file.lines() {
        let (id, misc) = x.split_at(x.find('.').unwrap());
        if String::from(id) == link.to_owned() {
            has_link = true;
            break;
        } else {
            continue;
        };
    }

    Ok(has_link)
}

pub const STORAGE_LOCATION: &'static str = env!("STORAGE");

pub fn store_link(link: &LinkRequest) -> Result<(), SaveError> {
    match read_to_string(STORAGE_LOCATION) {
        Ok(file) => {
            if let Ok(does_exist) = already_exists(&link.id, file.clone()) {
                if does_exist == true {
                    Err(SaveError::AlreadyExists)
                } else {
                    if let Ok(()) = write(
                        STORAGE_LOCATION,
                        format!(
                            "{}\n{}.{}.{}",
                            file,
                            link.id,
                            link.url.replace(".", "!"),
                            link.passcode
                        ),
                    ) {
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

// fn store_code(id: &String, code: String) -> Result<(), SaveError> {}

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
