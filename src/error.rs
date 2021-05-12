use failure::{Context, Backtrace, Fail};
use std::sync::PoisonError;

#[derive(Debug)]
pub struct RocksError {
    inner: Context<RocksErrorKind>
}

#[derive(Debug, Fail)]
pub enum RocksErrorKind {
    #[fail(display = "RocksDB error")]
    RError(#[cause] rocksdb::Error),
    #[fail(display = "Comb error")]
    CombError(String),
    #[fail(display = "SysDB error: {}", _0)]
    SysDB(String),
}

impl Fail for RocksError {
    fn cause(&self) -> Option<&dyn Fail> {
        self.inner.cause()
    }
    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl std::fmt::Display for RocksError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.inner, f)
    }
}

impl From<rocksdb::Error> for RocksError {
    fn from(err: rocksdb::Error) -> RocksError {
        RocksError {
            inner: Context::new(
                RocksErrorKind::RError(err)
            )
        }
    }
}

impl<T> From<PoisonError<T>> for RocksError {
    fn from(err: PoisonError<T>) -> RocksError {
        RocksError {
            inner: Context::new(
                RocksErrorKind::CombError(err.to_string())
            )
        }
    }
}

impl From<String> for RocksError {
    fn from(err: String) -> RocksError {
        RocksError {
            inner: Context::new(
                RocksErrorKind::CombError(err)
            )
        }
    }
}