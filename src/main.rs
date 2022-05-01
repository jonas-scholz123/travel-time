use anyhow::Result;
use api::{
    client::{Client, TFLClient},
    model::{
        stops_request::StopsByModeRequest,
        stops_response::{StopPointMode, StopsResponse},
    },
};

mod api;
pub mod utils;

#[tokio::main]
async fn main() {
    let client = TFLClient::new("7fa56d767da04461a225dfe82d34ef51").unwrap();
    let request = StopsByModeRequest::new(vec![StopPointMode::Dlr]);
    let response: Result<StopsResponse> = client.query(&request).await;

    match response {
        Ok(_) => {
            println!("Val");
        }
        Err(e) => println!("{}", e),
    }
}
