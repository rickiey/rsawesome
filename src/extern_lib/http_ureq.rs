
pub fn use_ureq() {

    let tm  = chrono::Utc::now();
    let resp = ureq::post("http://112.13.172.72:59000/penalty_msg")
        .send_json(ureq::json!(PenaltyMsg{
            from_addr: "f0123261".to_string(),
            amount: 0.to_string(),
            to_addr: "f099".to_string(),
            height: 6666,
            call_function: "call_function".to_string(),
            sub_cause: "sub_cause".to_string(),
            time_at: chrono::Local::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
        }));

    if resp.is_err() {
        println!("Penalty not sent to API  {}", resp.err().unwrap());
    } else {
        println!("{} {}",resp.as_ref().unwrap().status(), resp.unwrap().into_string().unwrap());
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PenaltyMsg {
    pub to_addr: String,
    pub from_addr: String,
    pub height: i64,
    pub amount: String,
    pub time_at: String,
    pub call_function: String,
    pub sub_cause: String,
}