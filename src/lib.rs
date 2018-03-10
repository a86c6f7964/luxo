use std::{io, result, str};
use std::io::Read;
mod simple;
pub use simple::open_simple;
mod memory;
pub use memory::open_memory;

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

pub trait Luxo {
    fn read(&self, key: &[u8], read_value: &Fn(&mut Read) -> usize) -> Result<Option<usize>>;
    fn write(&mut self, key: &[u8], value: &mut Read) -> Result<u64>;
}

pub fn stats(folder: &String) {
    println!("stats {}", folder)
}
