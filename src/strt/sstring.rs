pub fn sts() {
    create_str()
}

fn create_str() {
    let mut s1 = "s1".to_string();
    let s2 = String::from("s2");

    let mut s3 = String::new();
    s3.push('s');
    s3.push_str("3");
    s1 = s1 + &s2;

    println!("{} {} {}", s1, s2, s3);
    let mut my_name = "Pascal".to_string();
    my_name.push_str(" Precht");

    let _last_name = &my_name[7..];

    let s = String::from("hello中文");
    for c in s.chars() {
        print!("{}", c);
    }

    print!("\n");

    let f = format!("{}", "format string");
    println!("{}", first_word(&f));

    let _sss = &f[2..3];

    let x = 5;
    let y = x;
    println!("{} {}", x, y);

    let x1 = "asdasfasd".to_string();
    let y1 = &x1;
    println!("{} {}", x1, y1);
}

fn first_word(s: &String) -> String {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return (&s[0..i]).to_string();
        }
    }

    (&s[..]).to_string()
}
