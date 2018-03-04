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

pub fn open_simple(folder: &String) -> Result<Box<Luxo>> {
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

impl Luxo for SimpleLuxo {
    // https://bryce.fisher-fleig.org/blog/strategies-for-returning-references-in-rust/index.html
    fn read(&self, key: &[u8]) -> Result<Option<Box<Read>>> {
        let k = from_utf8(&key)?;
        let mut key_path = self.folder.to_path_buf();
        key_path.push(format!("{}.key", k));

        let file = File::open(key_path)?;
        let reader = BufReader::new(file);
        Ok(Some(Box::new(reader)))
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
