use std::{io, result, str};
use std::io::{copy, BufRead, BufReader, Read, Write};
use std::path::{Path, PathBuf};

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

use std::fs;
use std::fs::File;

pub fn open_simple(folder: &String) -> Result<Box<Luxo<File>>> {
    let path = Path::new(folder);
    if !path.is_dir() {
        fs::create_dir(&path)?;
    }

    let path = fs::canonicalize(path)?;
    Ok(Box::new(SimpleLuxo { folder: path }))
}

#[derive(Debug)]
struct SimpleLuxo {
    folder: PathBuf,
}

impl Luxo<File> for SimpleLuxo {
    // https://bryce.fisher-fleig.org/blog/strategies-for-returning-references-in-rust/index.html
    fn read(&self, key: &[u8]) -> Result<BufReader<File>> {
        let k = str::from_utf8(&key)?;
        let mut key_path = self.folder.to_path_buf();
        key_path.push(format!("{}.key", k));

        let file = File::open(key_path)?;
        let reader = BufReader::new(file);
        Ok(reader)
    }

    fn write(&self, key: &[u8], value: &mut BufRead) -> Result<u64> {
        let k = str::from_utf8(&key)?;

        let mut temp_path = self.folder.to_path_buf();
        temp_path.push(format!("{}.key.tmp", k));
        let mut end_path = self.folder.to_path_buf();
        end_path.push(format!("{}.key", k));

        let len;
        {
            let mut file = File::create(temp_path.as_path())?;
            len = copy(value, &mut file)?;
            file.flush()?;
            file.sync_all()?;
        }

        // out of scope so closed
        fs::rename(temp_path, end_path)?;

        // todo: folder fsync
        // https://lwn.net/Articles/457667/

        Ok(len)
    }
}