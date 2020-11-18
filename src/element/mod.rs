pub mod qtfile_datetime;
pub mod qtfile_matrix;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ElementParseError {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}
