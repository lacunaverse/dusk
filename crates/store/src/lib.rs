use util::LinkRequest;
use sled::Config;

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

pub fn store_link(link: LinkRequest) -> Result<(), SaveError> {
    let config = Config::default().path("./links");

    if let Ok(db) = config.open() {
        if let Ok(_) = db.get(link.id.to_owned()) {
            Err(SaveError::AlreadyExists)
        } else {
            if let Ok(_) = db.insert(link.id.as_bytes(), link.url.to_owned().as_bytes()) {
                match store_code(&link.id, link.passcode) {
                    Ok(()) => Ok(()),
                    Err(error) => Err(error),
                }
            } else {
                Err(SaveError::WriteFailure)
            }
        }
    } else {
        Err(SaveError::OpenFailure)
    }
}

fn store_code(id: &String, code: String) -> Result<(), SaveError> {
    let config = Config::default().path("./passcodes");

    if let Ok(db) = config.open() {
        if let Ok(v) = db.get(id) {
            Err(SaveError::AlreadyExists)
        } else {
            if let Ok(_) = db.insert(id.as_bytes(), code.as_bytes()) {
                Ok(())
            } else {
                Err(SaveError::WriteFailure)
            }
        }
    } else {
        Err(SaveError::OpenFailure)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
