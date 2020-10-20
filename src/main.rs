use reqwest;
use serde::Deserialize;
use serde::export::fmt::Debug;

fn main() {
    let _ = today_in_history();
}

#[derive(Deserialize, Debug)]
struct HistoryResult {
    _id: String,
    title: String,
    pic: String,
    year: i32,
    month: i32,
    day: i32,
    des: String,
    lunar: String,
}

#[derive(Deserialize, Debug)]
struct History {
    result: Vec<HistoryResult>,
    reason: String,
    error_code: i32,
}

fn today_in_history() -> Result<(), Box<dyn std::error::Error>> {
    let base_url = "";
    let resp = reqwest::blocking::get(base_url)?.json::<History>()?;
    let res: Vec<String> = resp.result.into_iter().
        map(|r| format!("{}年{}月{}日", r.year, r.month, r.day)).collect();
    println!("{:#?}", res);
    Ok(())
}