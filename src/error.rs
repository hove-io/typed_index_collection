use crate::collection::Id;
use thiserror::Error;

#[derive(Error)]
/// Typed error for `typed_index_collection`.
/// ```
/// # use typed_index_collection::{Error, Id};
/// struct Object;
/// impl Id<Object> for Object {
///   fn id(&self) -> &str { "object_id" }
///   fn set_id(&mut self, _: String) { todo!() }
/// }
/// let error = Error::IdentifierAlreadyExists(Object);
/// let msg = format!("{:?}", error);
/// // Output:
/// // IdentifierAlreadyExists Error { id: "object_id", type: "Object" }
/// # assert!(msg.contains("IdentifierAlreadyExists"));
/// # assert!(msg.contains("object_id"));
/// # assert!(msg.contains("Object"));
/// ```
pub enum Error<T: Id<T>> {
    /// This error occurs when an identifier already exists.
    #[error("identifier {} already exists", .0.id())]
    IdentifierAlreadyExists(T),
}

impl<T: Id<T>> std::fmt::Debug for Error<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IdentifierAlreadyExists(obj) => f
                .debug_struct("IdentifierAlreadyExists Error")
                .field("id", &obj.id())
                .field("type", &std::any::type_name::<T>())
                .finish(),
        }
    }
}
