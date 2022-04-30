use api::{
    client::{Client, TFLClient},
    model::{journey::JourneyRequest, journey_response::JourneyPlannerResult},
};

mod api;

#[tokio::main]
async fn main() {
    let client = TFLClient::new("7fa56d767da04461a225dfe82d34ef51").unwrap();

    let mut journey = JourneyRequest::default();
    journey.from = "w67qh".into();
    journey.to = "sw72bb".into();
    let a = client
        .query::<JourneyPlannerResult, JourneyRequest>(&journey)
        .await;
    match a {
        Ok(val) => {
            let b = val;
        } //println!("{}", val),
        Err(e) => println!("{}", e),
    }
}
