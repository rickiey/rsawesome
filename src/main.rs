#![feature(once_cell)]

mod algorithm;
mod collections;
mod enume;
mod extern_lib;
mod fmt_print;
mod leet_code;
mod mysqlcli;
mod strt;
mod struct_trait;
mod future_async;

#[macro_use]
extern crate serde_derive;

use futures::executor::block_on;
use futures::future::join;
use crate::algorithm::sort;

#[warn(unused_imports)]
use crate::strt::sstring::*;
use collections::coll;
use fmt_print::print_std::prints as ps;
use struct_trait::strait::{use_trait, MinMax, Point2D};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error +'static>>{

    let loglvl = env::var("RUST_LOG").unwrap_or("warn".to_string());
    println!("{}", loglvl);
    // let f = future_async::use_future();
    // extern_lib::rusqlite::use_sqlite();
    extern_lib::trace_opentelemetry::use_opentelemetry();
    extern_lib::http_ureq::use_ureq();
    // let rs = extern_lib::sqlxlite::use_sqlx().await?;

    // let a = block_on(extern_lib::sqlxlite::use_sqlx());

    // println!("{:?}", rs.len());
    // join(a, f).await;
    Ok(())
    // extern_lib::json_parse::use_serde_json();
    // leet_code::container_moster_water::test_001();
    // algorithm::sort::quick::use_quick_sort();
    // return;

    // let asfasd: Option<i32> = None;

    // print!("{}", asfasd.unwrap().expect("asfasd"));

    // extern_lib::req::use_reqwest();
    // max_area(vec![2, 3, 1, 5, 7]);
    //
    // let p = Person {
    //     job: Some(Job {
    //         phone_number: Some(PhoneNumber {
    //             area_code: None,
    //             number: 439222222,
    //         }),
    //     }),
    // };
    // p.work_phone_area_code().map(|f| println!("{}", f));
    //
    // // return;
    //
    // let _m1 = MinMax(0, 0);
    // let _m2 = Point2D { x: 0.0, y: 0.0 };
    // let _x = use_trait;
    // // _x();
    // ps();
    // sts();
    // enume::enumerate::use_enum();

    // println!("{}", _m1::type_name());

    // coll::use_vec();
    // coll::use_map();
    // use_trait()
    // mysqlcli::mycli::use_mysql()
}

struct Person {
    job: Option<Job>,
}

#[derive(Clone, Copy)]
struct Job {
    phone_number: Option<PhoneNumber>,
}

#[derive(Clone, Copy)]
struct PhoneNumber {
    area_code: Option<u8>,
    number: u32,
}

impl Person {
    // 获取此人的工作电话号码的区号（如果存在的话）。
    fn work_phone_area_code(&self) -> Option<u8> {
        // 没有`？`运算符的话，这将需要很多的嵌套的 `match` 语句。
        // 这将需要更多代码——尝试自己编写一下，看看哪个更容易。
        self.job?.phone_number?.area_code
    }
}

#[allow(unused)]
pub fn max_area(height: Vec<i32>) -> i32 {
    let mut l: i32 = 0;
    let mut r: i32 = height.len() as i32 - 1;
    let mut max_a = 0;
    while l < r {
        max_a = std::cmp::max(
            max_a,
            std::cmp::min(
                height.get(l as usize).unwrap(),
                height.get(r as usize).unwrap(),
            ) * (r - l),
        );
        if height.get(l as usize).unwrap() < height.get(r as usize).unwrap() {
            l += 1;
        } else {
            r -= 1;
        }
    }
    return max_a;
}

#[test]
fn test_max_areas() {
    assert_eq!(max_area(vec![1, 8, 7]), 7);
}

mod test_mod {
    #[test]
    fn test_max_area() {
        assert_eq!(super::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    }
}
