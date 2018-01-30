use std::{io, result, str};
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::Write;
use std::io::Read;
use std::io::BufReader;
use std::fs;

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

type Result<T> = result::Result<T, Error>;

pub trait Luxo<T: Read> {
    fn read(&self, key: &[u8]) -> Result<BufReader<T>>;
    fn write(&self, key: &[u8], value: &[u8]) -> Result<usize>;
}

pub fn open_with_folder(folder: String) -> Result<Box<Luxo<File>>> {
    let path = fs::canonicalize(Path::new(&folder))?;

    if !path.is_dir() {
        fs::create_dir(&path)?;
    }

    Ok(Box::new(FolderBackedLuxo { folder: path }))
}

#[derive(Debug)]
pub struct FolderBackedLuxo {
    folder: PathBuf,
}

pub trait Callback {
    fn with_u8(&self, value: &[u8]) -> ();
}

impl Luxo<File> for FolderBackedLuxo {
    // https://bryce.fisher-fleig.org/blog/strategies-for-returning-references-in-rust/index.html
    fn read(&self, key: &[u8]) -> Result<BufReader<File>> {
        let k = str::from_utf8(&key)?;
        let mut key_path = self.folder.to_path_buf();
        key_path.push(format!("{}.key", k));

        let file = File::open(key_path)?;
        let reader = BufReader::new(file);
        Ok(reader)
    }

    fn write(&self, key: &[u8], value: &[u8]) -> Result<usize> {
        let k = str::from_utf8(&key)?;

        let mut temp_path = self.folder.to_path_buf();
        temp_path.push(format!("{}.key.tmp", k));
        let mut end_path = self.folder.to_path_buf();
        end_path.push(format!("{}.key", k));

        {
            let mut file = File::create(temp_path.as_path())?;
            file.write_all(value)?;
            file.flush()?;
            file.sync_all()?;
        }

        // out of scope so closed
        fs::rename(temp_path, end_path)?;

        // todo: folder fsync
        // https://lwn.net/Articles/457667/

        Ok(value.len())
    }
}
