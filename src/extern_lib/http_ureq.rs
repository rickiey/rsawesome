
pub fn use_ureq() {
    let resp = ureq::post("http://127.0.0.1:8080/ping")
        .send_json(ureq::json!({
          "name": "martin",
          "rust": true
      }));

    if let Ok(resp) = resp {
        println!("{}", resp.status());
    }
}