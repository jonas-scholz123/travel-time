#![feature(type_alias_impl_trait)]
use anyhow::Result;
use api::{
    client::{Client, TFLClient},
    model::{
        stops_request::StopsByModeRequest,
        stops_response::{StopPointMode, StopsResponse},
    },
};
use serde_json::to_string;

mod api;
pub mod utils;

#[tokio::main]
async fn main() {
    let client = TFLClient::new("7fa56d767da04461a225dfe82d34ef51").unwrap();
    let request = StopsByModeRequest::new(vec![StopPointMode::Dlr]);
    let response: Result<StopsResponse> = client.query(&request).await;

    let b = vec![StopPointMode::Dlr, StopPointMode::Tube];
    let a = b
        .iter()
        .map(|c| {
            // Parsed as json string ""enum"". Remove first and last.
            let string = to_string(c).unwrap();
            let mut chars = string.chars();
            chars.next();
            chars.next_back();
            chars.as_str().to_string()
        })
        .collect::<Vec<String>>()
        .join(",");
    println!("{}", a);

    match response {
        Ok(_) => {
            println!("Val");
        }
        Err(e) => println!("{}", e),
    }
}
