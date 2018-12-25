use failure::{Fail, Error};

#[derive(Fail, Debug)]
pub enum FileError {
    #[fail(display = "File may be corrupted, reason: {}", _0)]
    Corrupted(String)
}

//noinspection RsTypeCheck
pub fn error_invalid_checksum() -> Error {
    Error::from(FileError::Corrupted("Invalid checksum".to_owned()))
}

pub fn error_corrupted_block() -> Error {
    Error::from(FileError::Corrupted("cannot read the whole block".to_owned()))
}