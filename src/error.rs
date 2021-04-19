use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum ChgError {
    #[error("I/O error: {0}")]
    IOError(std::io::Error),

    #[error("File does not look like a markdown record of changes")]
    UnrecognizedChangelog(),
}

impl From<std::io::Error> for ChgError {
    fn from(e: std::io::Error) -> Self {
        Self::IOError(e)
    }
}
