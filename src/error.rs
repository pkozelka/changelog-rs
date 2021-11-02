use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum ChgError {
    #[error("I/O error: {0}")]
    IOError(std::io::Error),

    #[error("File does not look like a markdown record of changes")]
    UnrecognizedChangelog(),

    #[error(
        "Missing separator '- ' between version and timestamp in release section header ('{0}')"
    )]
    MissingVersionDateSeparator(String),

    #[error("Invalid version ID ('{0}') in section header ('{1}')")]
    InvalidVersionID(String, String),

    #[error("Missing timestamp in release section header ('{0}')")]
    MissingTimestamp(String),

    #[error("Invalid timestamp in release section header ('{0}'): {1}")]
    InvalidTimestamp(String, String),

    #[error("Cannot deserialize config: {0}")]
    ConfigReadError(String),

    #[error("Cannot serialize config")]
    ConfigWriteError,

    #[error("ERROR: {0}")]
    Other(String),
}

impl From<std::io::Error> for ChgError {
    fn from(e: std::io::Error) -> Self {
        Self::IOError(e)
    }
}
