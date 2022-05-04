use std::fs::create_dir;

use anyhow::Result;
use api::{
    client::{Client, TFLClient},
    model::{
        line_request::LinesByModeRequest,
        line_response::LinesResult,
        stops_response::{StopPoint, TransportMode},
    },
};
use chrono::NaiveTime;
use db::mongo_loader::Loader;
use graph::graph::TflGraph;
use mongodb::options::ClientOptions;
use strum::IntoEnumIterator;

use crate::{api::model::direct_connection::DirectConnection, db::mongo_repo::MongoRepository};

mod api;
mod db;
pub mod graph;
pub mod utils;

#[tokio::main(worker_threads = 30)]
async fn main() {
    let result = load(LoadOptions {
        load_routes: false,
        load_stops: false,
        load_segments: false,
        load_timetables: true,
    });

    let printstr = match result.await {
        Ok(_) => "Load completed successfully.".into(),
        Err(e) => e.to_string(),
    };
    println!("{}", printstr);

    let result = build_graph();

    let printstr = match result.await {
        Ok(_) => "Graph built successfully.".into(),
        Err(e) => e.to_string(),
    };

    println!("{}", printstr);
}

async fn build_graph() -> Result<()> {
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
    client_options.app_name = Some("TravelTime".to_string());

    // Get a handle to the deployment.
    let mongo_client = mongodb::Client::with_options(client_options)?;
    let connection_repo = MongoRepository::<DirectConnection>::new(&mongo_client);
    let station_repo = MongoRepository::<StopPoint>::new(&mongo_client);
    let graph = TflGraph::from_repos(&connection_repo, &station_repo).await?;
    let scores = graph.time_dependent_dijkstra("490004733C".into(), NaiveTime::from_hms(10, 0, 0));
    println!("{:#?}", scores);
    Ok(())
}

async fn get_lines() -> Result<()> {
    let mut tfl_client = TFLClient::new("7fa56d767da04461a225dfe82d34ef51").unwrap();

    let all_modes: Vec<TransportMode> = TransportMode::iter().collect();
    let request = LinesByModeRequest::new(all_modes);
    let result = tfl_client.query::<LinesResult, _>(&request).await?;
    Ok(())
}

struct LoadOptions {
    load_stops: bool,
    load_routes: bool,
    load_segments: bool,
    load_timetables: bool,
}

async fn load(options: LoadOptions) -> Result<()> {
    let mut tfl_client = TFLClient::new("7fa56d767da04461a225dfe82d34ef51").unwrap();
    // Parse a connection string into an options struct.
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
    client_options.app_name = Some("TravelTime".to_string());

    // Get a handle to the deployment.
    let mongo_client = mongodb::Client::with_options(client_options)?;
    println!("Client OK.");

    let mut loader = Loader::new(&mut tfl_client, &mongo_client);

    if options.load_stops {
        println!("Loading stops.");
        loader.load_stops().await?;
        println!("Loaded stops.");
    }

    if options.load_routes {
        println!("Loading routes.");
        loader.load_routes().await?;
        println!("Loaded routes.");
    }

    if options.load_segments {
        println!("Loading segments.");
        loader.load_segments().await?;
        println!("Loaded segments.");
    }

    if options.load_timetables {
        /*
        MongoRepository::<DirectConnection>::new(&mongo_client)
            .collection
            .drop(None)
            .await?;
        */
        println!("Loading timetables.");
        loader.load_timetables().await?;
        println!("Loaded timetables.");
    }
    Ok(())
}
