/// Implements the Id trait for an object. For example, if an object Animal has
/// a field 'species_id' which is a pointer toward another Object of type
/// Species, you can implements Id with the following example. You can also
/// implement Id for Animal itself (the identifier of an Animal is its own
/// field that must be named `id`).
/// ```
/// # use typed_index_collection::impl_id;
/// # fn main() {
/// struct Species {
///   id: String,
///   name: String,
/// }
/// struct Animal {
///   id: String,
///   name: String,
///   species_id: String,
/// }
/// impl_id!(Animal, Species, species_id);
/// impl_id!(Animal);
/// # }
/// ```
#[macro_export]
macro_rules! impl_id {
    ($ty:ty, $gen:ty, $id: ident) => {
        impl typed_index_collection::Id<$gen> for $ty {
            #[allow(clippy::misnamed_getters)]
            fn id(&self) -> &str {
                &self.$id
            }

            fn set_id(&mut self, id: std::string::String) {
                self.$id = id;
            }
        }
    };
    ($ty:ty) => {
        impl_id!($ty, $ty, id);
    };
}

/// Implement trait `WithId` automatically for a type.
///
/// The type must implement `Default` and have at least 2 fields: `id` and
/// `name`. Both `id` and `name` will be set with the value of the input
/// parameter `id`.
/// ```
/// # use typed_index_collection::{impl_with_id, WithId};
/// # fn main() {
/// #[derive(Default)]
/// struct Animal {
///   id: String,
///   name: String,
///   species: String,
/// }
/// impl_with_id!(Animal);
/// let animal = Animal::with_id("cat");
/// assert_eq!("cat", animal.id);
/// assert_eq!("cat", animal.name);
/// assert_eq!("", animal.species);
/// # }
/// ```
#[macro_export]
macro_rules! impl_with_id {
    ($ty:ty) => {
        impl typed_index_collection::WithId for $ty {
            // This warning occurs when the type only has id and name members and no other
            #[allow(clippy::needless_update)]
            fn with_id(id: &str) -> Self {
                Self {
                    id: id.to_owned(),
                    name: id.to_owned(),
                    ..Default::default()
                }
            }
        }
    };
}
