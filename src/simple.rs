use std::fs;
use std::fs::File;
use std::path::Path;
use super::Result;
use super::Luxo;
use std::path::PathBuf;
use std::io::Read;
use std::io::BufReader;
use std::io::copy;
use std::io::Write;
use str::from_utf8;

extern crate fs2;
use self::fs2::FileExt;

pub fn open_simple(folder: &String) -> Result<Box<Luxo>> {
    let path = Path::new(folder);
    if !path.is_dir() {
        fs::create_dir(&path)?;
    }

    let path = fs::canonicalize(path)?;
    Ok(Box::new(SimpleLuxo::open(path)))
}

#[derive(Debug)]
struct SimpleLuxo {
    folder: PathBuf,
    lock_file: File,
}

impl SimpleLuxo {
    fn open(folder: PathBuf) -> SimpleLuxo {
        let mut key_path = folder.to_path_buf();
        key_path.push(".lock");
        let key_path_str: &str = key_path
            .to_str()
            .expect(&format!("unable to create lock string"));
        let lock_file: File =
            File::create(&key_path).expect(&format!("unable to open lock [{}]", key_path_str));
        lock_file
            .try_lock_exclusive()
            .expect(&format!("unable to lock [{}]", key_path_str));

        SimpleLuxo { folder, lock_file }
    }
}

impl Luxo for SimpleLuxo {
    // https://bryce.fisher-fleig.org/blog/strategies-for-returning-references-in-rust/index.html
    fn read(
        &self,
        key: &[u8],
        read_value: &mut FnMut(&mut Read) -> usize,
    ) -> Result<Option<usize>> {
        let k = from_utf8(&key)?;
        let mut key_path = self.folder.to_path_buf();
        key_path.push(format!("{}.key", k));

        let file = File::open(key_path)?;
        let mut reader: BufReader<File> = BufReader::new(file);
        Ok(Some(read_value(&mut reader)))
    }

    fn write(&mut self, key: &[u8], value: &mut Read) -> Result<u64> {
        let k = from_utf8(&key)?;

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
