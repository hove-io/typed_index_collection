use thiserror::Error;

#[derive(Error)]
/// Typed error for `typed_index_collection`.
pub enum Error<T: crate::collection::Id<T>> {
    /// This error occurs when an identifier already exists.
    #[error("identifier {} already exists", .0.id())]
    IdentifierAlreadyExists(T),
}

impl<T: crate::collection::Id<T>> std::fmt::Debug for Error<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IdentifierAlreadyExists(obj) => f
                .debug_struct("IdentifierAlreadyExists Error")
                .field("id", &obj.id())
                .finish(),
        }
    }
}
