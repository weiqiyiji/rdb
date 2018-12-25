use failure::Fail;

#[derive(Fail, Debug)]
pub enum FileError {
    #[fail(display = "File may be corrupted, reason: {}", _0)]
    Corrupted(String)
}