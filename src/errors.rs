use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("I/O error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("error while deserializing TOML: {0}")]
    DeserError(#[from] toml::de::Error),
}
