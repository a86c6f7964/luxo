mod luxo;
use std::str;

pub fn stats(folder: String) {
    println!("stats {}", folder)
}

pub fn example(folder: String) {
    let luxo = luxo::open_with_folder(folder).unwrap();
    for i in 1..20 {
        let written = luxo.write(
            format!("test{}", i).as_bytes(),
            format!("value {}", i).as_bytes(),
        ).unwrap();
        println!("able to write [test{}] length {}", i, written)
    }
    for i in 1..20 {
        let mut value = Vec::new();
        let read = luxo.read(format!("test{}", i).as_bytes(), &mut value).unwrap();
        println!("able to read [test{}] of L:{}[{}]", i, read, str::from_utf8(&value).unwrap())
    }
}
