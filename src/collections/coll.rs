use std::collections::HashMap;

pub fn use_vec() {
    let v = vec![1, 23, 4];
    for i in v.iter() {
        println!("{}", i);
    }

    let mut vv: Vec<&str> = vec!["a", "b", "c"];
    vv[2] = "vvvv";
    println!(
        "{:?} {}",
        match vv.get(3) {
            None => "",
            _ => vv.get(3).unwrap(),
        },
        vv[2]
    );

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![1, 2, 3];
    for i in &mut v {
        *i *= *i;
    }

    println!("{:?}", v);

    let v = vec![1, 2, 3];
    for i in v.iter().enumerate() {
        println!("index: {}  value: {}", i.0, i.1);
    }
}

// pub fn print_type_of<T>(_: &T) -> () {
//     let type_name = unsafe { (*std::intrinsics::get_tydesc::<T>()).name };
//     println!("{}", type_name);
// }

pub fn use_map() {
    let mut mp: HashMap<String, i64> = HashMap::new();
    mp.insert(String::from("six"), 6);
    mp.insert("one".to_string(), 1);

    println!("{:?}", mp);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("{:?}", scores);
}
