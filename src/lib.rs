mod luxo;
use std::io::Read;

pub fn stats(folder: String) {
    println!("stats {}", folder)
}

pub fn example(folder: String) {
    let luxo = luxo::open_with_folder(folder).unwrap();

    for i in 1..20 {
        let written = luxo.write(
            format!("test{}", i).as_bytes(),
            &mut format!("value {}", i).as_bytes(),
        ).expect("unable to write");

        println!("able to write [test{}] length {}", i, written)
    }

    for i in 1..20 {
        let mut buf = luxo.read(format!("test{}", i).as_bytes())
            .expect("unable to find buffer");

        let mut value = String::new();
        let num: usize = buf.read_to_string(&mut value)
            .expect("unable to read to end");

        println!("able to read [test{}] of L:{}[{}]", i, num, value)
    }
}
