use super::Result;
use super::Luxo;
use std::io::Read;
use std::collections::HashMap;

pub fn open_memory() -> Result<Box<Luxo>> {
    Ok(Box::new(MemoryLuxo { int_map: HashMap::new() }))
}

#[derive(Debug)]
struct MemoryLuxo {
    int_map: HashMap<Vec<u8>, Vec<u8>>
}

impl Luxo for MemoryLuxo {
    fn read(&self, key: &[u8]) -> Result<Option<Box<Read>>> {
        //let option: Option<&Vec<u8>> = self.int_map.get(key);
        /*if let Some(val) = option {
            return Ok(Some(Box::new(val.as_slice())))
        } else {
            return Ok(None)
        }*/
        Ok(None)
    }

    fn write(&mut self, key: &[u8], value: &mut Read) -> Result<u64> {
        let mut r = Vec::new();
        value.read_to_end(&mut r)?;
        let l = r.len();
        self.int_map.insert(key.to_vec(), r);
        Ok(l as u64)
    }
}
