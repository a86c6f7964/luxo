use duration::Millis;
use std::time::Instant;
use luxo::open_simple;
use luxo::open_memory;

pub fn example(folder: &String, store: &String) {
    println!("open folder [{}]", folder);
    let mut luxo = match store.as_ref() {
        "simple" => {
            Ok(open_simple(folder).expect(&format!("unable to open [{}/{}]", folder, store)))
        }
        "memory" => Ok(open_memory().expect(&format!("unable to open [{}/{}]", folder, store))),
        _ => Err(format!("unknown store [{}]", store)),
    }.unwrap();

    let now = Instant::now();
    let num_keys = 2;
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
            if let Some(_) = luxo.read(key, &|buf| {
                let mut value = Vec::new();
                let res = buf.read_to_end(&mut value).expect("unable to read to end");
                assert_eq!(value[..], values.get(i).expect("unable to find value")[..]);
                res
            }).expect("unable to find buffer")
            {
            } else {
                panic!("unable to find key #{}", i)
            }
        } else {
            panic!("unable to find key #{}", i)
        }
    }

    println!(
        "took {}ms to read and assert key/vals",
        now.elapsed().as_millis()
    );
}
