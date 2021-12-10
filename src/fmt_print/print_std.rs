pub fn prints() {
    println!("{0} {a1}", "args02", a1 = "args01");
    let s = "2".parse::<f64>().unwrap();
    println!("{} {1}", 3.to_string(), s);
    println!("{} {:?}", "args02", "args01".to_string());
    println!("{number:>width$}", number = 1, width = 6);
    println!("{number:0>w$}", number = 1, w = 6);
}
