use super::Result;
use super::Luxo;
use std::io::Read;
use std::collections::HashMap;

type ByteString = Vec<u8>;

pub fn open_memory() -> Result<Box<Luxo>> {
    Ok(Box::new(MemoryLuxo {
        int_map: HashMap::new(),
    }))
}

#[derive(Debug)]
struct MemoryLuxo {
    int_map: HashMap<ByteString, ByteString>,
}

impl Luxo for MemoryLuxo {
    fn read(
        &self,
        key: &[u8],
        read_value: &mut FnMut(&mut Read) -> usize,
    ) -> Result<Option<usize>> {
        if let Some(val) = self.int_map.get(key) {
            return Ok(Some(read_value(&mut val.as_slice())));
        } else {
            return Ok(None);
        }
    }

    fn write(&mut self, key: &[u8], size: usize, value: &mut Read) -> Result<u64> {
        let mut r = Vec::new();
        value.read_to_end(&mut r)?;
        let l = r.len();
        self.int_map.insert(key.to_vec(), r);
        Ok(l as u64)
    }
}
