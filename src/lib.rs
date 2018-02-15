mod luxo;
use std::io::Read;

pub fn stats(folder: &String) {
    println!("stats {}", folder)
}

pub fn example(folder: &String, store: &String) {
    println!("open folder [{}]", folder);
    let luxo = match store.as_ref() {
        "simple" => Ok(luxo::open_with_folder(folder)
            .expect(&format!("unable to open [{}/{}]", folder, store))),
        _ => Err(format!("unknown store [{}]", store)),
    }.unwrap();

    for i in 1..2000 {
        let _written = luxo.write(
            format!("test{}", i).as_bytes(),
            &mut format!("value {}", i).as_bytes(),
        ).expect("unable to write");

        //println!("able to write [test{}] length {}", i, written)
    }

    for i in 1..2000 {
        let mut buf = luxo.read(format!("test{}", i).as_bytes())
            .expect("unable to find buffer");

        let mut value = String::new();
        let _num = buf.read_to_string(&mut value)
            .expect("unable to read to end");

        //println!("able to read [test{}] of L:{}[{}]", i, num, value)
    }
    println!("done reading/writing")
}
