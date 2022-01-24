extern crate reqwest;

use std::error;

// use reqwest;

pub fn use_reqwest() -> Result<i32, reqwest::Error> {
    // let b =  blocking::get("http://localhost:9090/").text();
    //
    // println!("{:?}", b);

    // let body = reqwest::blocking::get("http://localhost:9090/")?.text()?;

    let resp = reqwest::blocking::get("http://localhost:9090/");

    println!("body = {:?}", resp?.text().unwrap());

    Ok(4)
}
