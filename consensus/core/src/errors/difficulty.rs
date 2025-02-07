use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum DifficultyError {
    #[error("under min allowed window size ({0} < {1})")]
    UnderMinWindowSizeAllowed(usize, usize),

    #[error("min window timestamp is equal to the max window timestamp")]
    EmptyTimestampRange,
}

pub type DifficultyResult<T> = std::result::Result<T, DifficultyError>;
