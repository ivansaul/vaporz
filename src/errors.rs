use std::sync::{PoisonError, RwLockReadGuard, RwLockWriteGuard};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    LockError(String),

    #[error("{0}")]
    Errrr(#[from] color_eyre::eyre::ErrReport),
}

pub type Result<T> = std::result::Result<T, Error>;

impl<T> From<PoisonError<RwLockReadGuard<'_, T>>> for Error {
    fn from(err: PoisonError<RwLockReadGuard<'_, T>>) -> Self {
        Error::LockError(err.to_string())
    }
}

impl<T> From<PoisonError<RwLockWriteGuard<'_, T>>> for Error {
    fn from(err: PoisonError<RwLockWriteGuard<'_, T>>) -> Self {
        Error::LockError(err.to_string())
    }
}
