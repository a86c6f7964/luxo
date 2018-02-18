mod example;
pub mod duration;
pub mod simple;

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
use std::time::Instant;

pub fn example(folder: &String, store: &String) {
    println!("open folder [{}]", folder);
    let luxo = match store.as_ref() {
        "simple" => {
            Ok(open_simple(folder).expect(&format!("unable to open [{}/{}]", folder, store)))
        }
        _ => Err(format!("unknown store [{}]", store)),
    }.unwrap();

    let now = Instant::now();
    let num_keys = 10000;
    let mut keys: Vec<Vec<u8>> = Vec::with_capacity(num_keys);
    let mut values: Vec<Vec<u8>> = Vec::with_capacity(num_keys);
    for i in 1..num_keys {
        let key = format!("test{}", i);
        let value = format!("value {}", i);
        keys.push(key.into_bytes());
        values.push(value.into_bytes());
    }

    println!("took {}ms to build the strings", now.elapsed().as_millis());

    for i in 0..num_keys - 1 {
        if let Some(key) = keys.get(i) {
            if let Some(value) = values.get(i) {
                luxo.write(key, &mut &value[..]).expect("unable to write");
            }
        } else {
            panic!("unable to find key #{}", i)
        }
    }

    println!("took {}ms to write key/vals", now.elapsed().as_millis());
    let now = Instant::now();

    for i in 0..num_keys - 1 {
        if let Some(key) = keys.get(i) {
            let mut buf = luxo.read(key).expect("unable to find buffer");
            let mut value = Vec::new();
            buf.read_to_end(&mut value).expect("unable to read to end");
            assert_eq!(value[..], values.get(i).expect("unable to find value")[..])
        } else {
            panic!("unable to find key #{}", i)
        }
    }

    println!(
        "took {}ms to read and assert key/vals",
        now.elapsed().as_millis()
    );
}
use std::time::Duration;

pub trait Millis {
    fn as_millis(&self) -> u64;
}

impl Millis for Duration {
    fn as_millis(&self) -> u64 {
        return (self.as_secs() * 1_000) + (self.subsec_nanos() / 1_000_000) as u64;
    }
}
