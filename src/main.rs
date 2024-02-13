use std::collections::HashMap;

use reqwest::header::{HeaderMap, HeaderValue};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut headers = HeaderMap::new();
    let email = "derek.j.marion@gmail.com";
    headers.insert("X-RapidAPI-Key", HeaderValue::from_static("APIKEY"));
    headers.insert("X-RapidAPI_Host", HeaderValue::from_static("email-checker.p.rapidapi.com"));

    let mut params = HashMap::new();
    params.insert("email", email);

    let client = reqwest::Client::new();
    let response = client
        .get("https://email-checker.p.rapidapi.com/verify/v1")
        .headers(headers)
        .query(&params)
        .send()
        .await?;

    let body = response.text().await?;
    println!("{}", body);

    Ok(())
}
