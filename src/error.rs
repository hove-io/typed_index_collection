use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Identifier {0} already exists")]
    IdentifierAlreadyExists(String),
}
