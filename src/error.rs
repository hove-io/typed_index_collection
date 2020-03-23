use thiserror::Error;

#[derive(Error, Debug)]
/// Typed error for `collections`.
pub enum Error {
    /// This error occurs when an identifier already exists.
    #[error("identifier {0} already exists")]
    IdentifierAlreadyExists(String),
}
