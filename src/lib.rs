use std::{io, result, str};
use std::io::Read;

extern crate serde_json;
#[macro_use]
extern crate serde_derive;

mod simple;
pub use simple::open_simple;
mod memory;
pub use memory::open_memory;
mod wal;
pub use wal::open_wal;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Utf8Error(str::Utf8Error),
    String(String),
    JsonError(serde_json::Error),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<str::Utf8Error> for Error {
    fn from(err: str::Utf8Error) -> Error {
        Error::Utf8Error(err)
    }
}

impl From<String> for Error {
    fn from(err: String) -> Error {
        Error::String(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::JsonError(err)
    }
}

pub type Result<T> = result::Result<T, Error>;

pub trait Luxo {
    fn read(&self, key: &[u8], read_value: &mut FnMut(&mut Read) -> usize)
        -> Result<Option<usize>>;
    fn write(&mut self, key: &[u8], size: usize, value: &mut Read) -> Result<u64>;
}

pub fn stats(folder: &String) {
    println!("stats {}", folder)
}
