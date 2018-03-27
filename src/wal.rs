use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::path::Path;
use super::Result;
use super::Luxo;
use std::path::PathBuf;
use std::io::Cursor;
use std::io::Read;
use std::io::Write;
use std::io;
use serde_json;

extern crate core;
extern crate fs2;
extern crate memmap;
extern crate byteorder;

use self::fs2::FileExt;
use self::memmap::{MmapMut, MmapOptions};
use self::byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

pub fn open_wal(folder: &String) -> Result<Box<Luxo>> {
    let path = Path::new(folder);
    if !path.is_dir() {
        fs::create_dir(&path)?;
    }

    let path = fs::canonicalize(path)?;
    Ok(Box::new(WalLuxo::open(path)?))
}

#[derive(Debug)]
struct WalLuxo {
    lock_file: File,
    meta_path: PathBuf,
    meta: WalMeta,
    current_segment: Option<WalSegment>,
}

#[derive(Debug)]
struct WalSegment {
    name: String,
    last_flush: usize,
    mmap: MmapMut,
}

#[derive(Serialize, Deserialize, Debug)]
struct WalMeta {
    version: u64,
    current_segment_number: u64,
    block_size: u64,
}

impl WalLuxo {
    fn open(folder: PathBuf) -> Result<WalLuxo> {
        let mut lock_path = folder.to_path_buf();
        lock_path.push(".lock");
        let lock_file: File = File::create(&lock_path)?;
        lock_file.try_lock_exclusive()?;

        let mut meta_path = folder.to_path_buf();
        meta_path.push("meta.json");
        if !meta_path.is_file() {
            let meta = WalMeta{
                version: 0,
                current_segment_number: 0,
                block_size: 4096,
            };
            tmp_and_sync_write_json(&meta_path, &meta)?;
        };

        let file = File::open(&meta_path)?;
        Ok(WalLuxo {
            lock_file,
            meta_path,
            meta: serde_json::from_reader(&file)?,
            current_segment: None,
        })
    }
}

fn tmp_and_sync_write_json(meta_path: &PathBuf, meta: &WalMeta) -> Result<()> {
    let tmp = format!("{}.tmp", meta_path.to_str().unwrap());
    let meta_path_tmp = Path::new(&tmp);
    let mut meta_file_tmp = File::create(&meta_path_tmp)?;
    serde_json::to_writer_pretty(&meta_file_tmp, &meta)?;
    meta_file_tmp.flush()?;
    meta_file_tmp.sync_all()?;
    fs::rename(meta_path_tmp, meta_path)?;
    Ok(())
}

fn write_bytes(segment: &mut WalSegment, size: usize, value: &mut Read) -> Result<()> {
    let start = segment.last_flush;
    let end = start +4+ size;

    // todo: throw if out of bounds?
    // assert start is u64
    // assert size is u64
    // assert start + size in segment

    {
        // todo: does this actually allocate a new object?
        let mut buff: Cursor<&mut [u8]> = Cursor::new(&mut segment.mmap);
        buff.set_position(start as u64);
        buff.write_u32::<BigEndian>(size as u32)?;
        let written = io::copy(value, &mut buff)?;
        assert_eq!(written, size as u64);
    }
    segment.mmap.flush()?;
    segment.last_flush = end;
    return Ok(())
}

fn write_bytes_u8(segment: &mut WalSegment, value: &[u8]) -> Result<()> {
    let start = segment.last_flush;
    let size = value.len();
    let end = start + 4 + size;

    // todo: throw if out of bounds?
    // assert start is u64
    // assert size is u64
    // assert start + size in segment

    {
        // todo: does this actually allocate a new object?
        let mut buff: Cursor<&mut [u8]> = Cursor::new(&mut segment.mmap);
        buff.set_position(start as u64);
        buff.write_u32::<BigEndian>(size as u32)?;
        let written = io::copy(value, &mut buff)?;
        assert_eq!(written, size as u64);
    }
    segment.mmap.flush()?;
    segment.last_flush = end;
    return Ok(())
}

impl Luxo for WalLuxo {
    fn read(
        &self,
        key: &[u8],
        read_value: &mut FnMut(&mut Read) -> usize,
    ) -> Result<Option<usize>> {
        unimplemented!()
    }

    fn write(&mut self, key: &[u8], size: usize, value: &mut Read) -> Result<u64> {
        if let Some(ref mut current) = self.current_segment {
            println!("current segment {:?}", current);
            write_bytes(current, size, value)?;

        } else {
            let mut path = self.meta_path.to_owned();
            path.pop();
            path.push("segments");
            if !path.is_dir() {
                fs::create_dir(&path)?;
            }
            let name = format!("{:09}.wal", self.meta.current_segment_number);
            path.push(name[..3].to_owned());
            if !path.is_dir() {
                fs::create_dir(&path)?;
            }
            path.push(name[3..6].to_owned());
            if !path.is_dir() {
                fs::create_dir(&path)?;
            }
            path.push(&name);

            let file = OpenOptions::new().read(true).write(true).create(true).open(&path)?;
            file.set_len(self.meta.block_size)?;
            let mmap = unsafe {
                MmapOptions::new().map_mut(&file)?
            };
            mmap.flush()?;


            let mut current = WalSegment{
                name,
                last_flush: 0,
                mmap
            };
            println!("new segment {:?}", current);
            write_bytes_u8(&mut current, key)?;
            write_bytes(&mut current, size, value)?;
            self.current_segment = Some(current);
        };

        Ok(0)
    }
}
