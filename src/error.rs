use thiserror::Error;

#[derive(Error, Debug)]
/// Typed error for `typed_index_collection`.
pub enum Error {
    /// This error occurs when an identifier already exists.
    #[error("identifier {0} already exists")]
    IdentifierAlreadyExists(String),
}
