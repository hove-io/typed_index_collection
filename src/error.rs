use thiserror::Error;

#[derive(Error)]
/// Typed error for `typed_index_collection`.
pub enum Error<T> {
    /// This error occurs when an identifier already exists.
    #[error("identifier {0} already exists")]
    IdentifierAlreadyExists(String, T),
}

impl<T> std::fmt::Debug for Error<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IdentifierAlreadyExists(id, _) => f
                .debug_struct("IdentifierAlreadyExists Error")
                .field("id", &id)
                .finish(),
        }
    }
}
