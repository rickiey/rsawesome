#[derive(Debug)]
pub enum IpAddrKid {
    Loop,
    V4(String),
    V6(String),
}

impl IpAddrKid {
    pub fn print_ip(self) {
        match self {
            IpAddrKid::V4(s4) => {
                println!("{:?}", s4);
            }
            IpAddrKid::V6(s6) => {
                println!("{:?}", s6);
            }

            IpAddrKid::Loop => {
                println!("this is looplocal")
            }

            #[warn(unreachable_patterns)]
            _ => {
                println!("default no match anything");
            }
        }
    }
}

pub fn use_enum() {
    let loop_local = IpAddrKid::Loop;
    loop_local.print_ip();
    let v4 = IpAddrKid::V4(String::from("127.0.0.1"));
    v4.print_ip();

    let v6 = IpAddrKid::V6(String::from("127.0.0.1"));
    v6.print_ip();

    let o = Option::Some(5);

    let ssss = s(o);
    println!("sssss {}", ssss);
    let sn: Option<i32> = None;
    println!("sssss {}", s(sn));

    let some_u8_value = Some(0_u8);
    match some_u8_value {
        Some(3) => println!("three"),
        Some(0u8) => println!("eight {}", Some(0u8).unwrap()),
        _ => println!("none"),
    }

    let mut some1 = Some(2);
    if let Some(2) = some1 {
        some1 = Some(some1.unwrap() + 1)
    } else {
        println!("{}", some1.unwrap());
        return;
    }
    println!("{}", some1.map(mull).unwrap());
    super::super_fn();
}

fn mull(x: i32) -> i32 {
    return x * x;
}

fn s(op: Option<i32>) -> i32 {
    let ss = match op {
        Some(si) => si,
        None => 0,
    };

    return ss;
}
