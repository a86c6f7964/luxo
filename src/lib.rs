mod luxo;
use luxo::Luxo;

pub fn stats(folder: String) {
    println!("stats {}", folder)
}

pub fn example(folder: String) {
    let luxo = Luxo::open(folder).unwrap();
    for i in 1..20 {
        let written = luxo.write(
            format!("test{}", i).as_bytes(),
            format!("value {}\n", i).as_bytes()
        ).unwrap();
        println!("able to write [test{}] length {}", i, written)
    }
}
