/// Implements the Id trait for an object. For example, if an object Animal has
/// a field 'species_id' which is a pointer toward another Object of type
/// Species, you can implements Id with the following example. You can also
/// implement Id for Animal itself (the identifier of an Animal is its own
/// field that must be named `id`).
/// ```
/// # use collections::impl_id;
/// # fn main() {
/// struct Species {
///   id: String,
///   name: String,
/// }
/// struct Animal {
///   id: String,
///   name: String,
/// # #[macro_use]
///   species_id: String,
/// }
/// impl_id!(Animal, Species, species_id);
/// impl_id!(Animal);
/// # }
/// ```
#[macro_export]
macro_rules! impl_id {
    ($ty:ty, $gen:ty, $id: ident) => {
        impl collections::Id<$gen> for $ty {
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
