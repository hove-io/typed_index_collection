use thiserror::Error;

#[derive(Error, Debug)]
/// The specific type of an error.
pub enum Error {
    /// This error occurs when an identifier already exists.
    #[error("identifier {0} already exists")]
    IdentifierAlreadyExists(String),
}
