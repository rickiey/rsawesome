extern crate serde;
extern crate serde_json;


use serde_json::Value as jsonValue;

#[derive(Serialize, Deserialize, Debug)]
pub struct Res {
    #[serde(rename(serialize = "code", deserialize = "code"))]
    code: String,
}

pub fn use_serde_json() {
    let json_raw = r#"{"code":"ok"}"#;

    let res: jsonValue = serde_json::from_str(json_raw).expect(" 解析错误");

    println!("{}", res["code"].as_str().unwrap());

    let r: Res =serde_json::from_str(json_raw).unwrap();
    println!("{}", r.code);

    let rr :Res = Res {
        code: "non".to_string(),
    };

    let a = serde_json::to_string(&rr).unwrap();

    println!("{}", a.to_string());

}
