mod luxo;
use luxo::Luxo;
use std::str;

pub fn stats(folder: String) {
    println!("stats {}", folder)
}

pub fn example(folder: String) {
    let luxo = Luxo::open(folder).unwrap();
    for i in 1..20 {
        let written = luxo.write(
            format!("test{}", i).as_bytes(),
            format!("value {}", i).as_bytes(),
        ).unwrap();
        println!("able to write [test{}] length {}", i, written)
    }
    for i in 1..20 {
        let read: String = luxo.read(format!("test{}", i).as_bytes(), |bytes| {
            Ok(str::from_utf8(bytes)?.to_owned())
        }).unwrap();
        println!("able to read [test{}] of [{}]", i, read)
    }
}
