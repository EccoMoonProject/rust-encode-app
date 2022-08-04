extern crate base64;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use base64::{decode, encode};
use reqwest;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::stdin;
use std::str;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    let forms = client
        .get("https://api-missions.moonie.io/wallet/0xEe9a4bf9835f4e5AF36522B04b317E913F2213EC")
        .header("Accept", "text/plain")
        .timeout(Duration::from_secs(3))
        .send()
        .await?
        .text()
        .await?;
    // println!("{:}", doge);
    fs::write("src/inp.json", forms).expect("Unable to write file");
    Ok(())
}