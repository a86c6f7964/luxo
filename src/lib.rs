use std::{io, result, str};
use std::io::{BufRead, BufReader, Read};
mod simple;
pub use simple::open_simple;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Utf8Error(str::Utf8Error),
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

pub type Result<T> = result::Result<T, Error>;

pub trait Luxo<R: Read> {
    fn read(&self, key: &[u8]) -> Result<BufReader<R>>;
    fn write(&self, key: &[u8], value: &mut BufRead) -> Result<u64>;
}

pub fn stats(folder: &String) {
    println!("stats {}", folder)
}
