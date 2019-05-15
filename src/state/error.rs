#[derive(Debug, Fail)]
pub enum ErrorKind {
    #[fail(display = "failed to mutex access")]
    Poison,
    #[fail(display = "failed to srde struct to json")]
    SerdeJson,
}

/* ----------- failure boilerplate ----------- */

use std::fmt;
use std::fmt::Display;
use failure::{Backtrace, Context, Fail};

#[derive(Debug)]
pub struct Error {
    inner: Context<ErrorKind>,
}

impl Fail for Error {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

impl Error {
    pub fn new(inner: Context<ErrorKind>) -> Error {
        Error { inner }
    }

    pub fn kind(&self) -> &ErrorKind {
        self.inner.get_context()
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Error {
            inner: Context::new(kind),
        }
    }
}

impl From<Context<ErrorKind>> for Error {
    fn from(inner: Context<ErrorKind>) -> Error {
        Error { inner }
    }
}

/* error translate */
use std::sync;

#[derive(Debug, Fail)]
#[fail(display = "Mutex access was poisoned")]
pub struct MyPoisonError {
    backtrace: Backtrace
}

impl MyPoisonError {
    pub fn new() -> MyPoisonError {
        MyPoisonError {
            backtrace: Backtrace::new()
        }
    }
}

//for op-?, "auto" type conversion
impl<T> From<sync::PoisonError<T>> for MyPoisonError {
    fn from(_: sync::PoisonError<T>) -> Self {
        MyPoisonError::new()
    }
}

impl From<MyPoisonError> for Error {
    fn from(error: MyPoisonError) -> Error {
        Error {
            inner: error.context(ErrorKind::Poison),
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Error {
        Error {
            inner: error.context(ErrorKind::SerdeJson),
        }
    }
}