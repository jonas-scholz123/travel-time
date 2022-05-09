use anyhow::Result;
use chrono::NaiveTime;
use db::mongo_loader::Loader;
use graph::tfl_graph::TflGraph;
use mongodb::options::ClientOptions;
use tfl::{client::TFLClient, model::stops_response::StopPoint};

use crate::{
    db::{data_fixer::DataFixer, mongo_repo::MongoRepository},
    national_rail::{s3::NationalRailS3, timetable_loader::TimetableLoader},
    tfl::model::direct_connection::DirectConnection,
};

mod db;
pub mod graph;
pub mod national_rail;
mod tfl;
pub mod utils;

#[tokio::main]
async fn main() {
    let result = load(LoadOptions {
        load_routes: false,
        load_stops: false,
        load_segments: false,
        load_timetables: true,
        fix_timetables: false,
        fix_stoppoints: false,
        load_national_rail_data: false,
        load_national_rail: false,
    });

    let printstr = match result.await {
        Ok(_) => "Load completed successfully.".into(),
        Err(e) => e.to_string(),
    };
    println!("{}", printstr);
}

async fn build_graph() -> Result<()> {
    let path = "./cache/graph.json";

    println!("Building graph");
    let cache = false;

    let graph = match !cache {
        true => build_graph_from_scratch().await?,
        false => match TflGraph::from_cache(path).await {
            Ok(g) => g,
            Err(e) => {
                println!("Falling back to rebuilding graph because {}", e);
                build_graph_from_scratch().await?
            }
        },
    };

    if cache {
        graph.cache(path).await?;
    }

    println!("Computing dijkstra's algorithm.");
    let scores = graph.time_dependent_dijkstra("490004733C".into(), NaiveTime::from_hms(10, 0, 0));
    println!("{:#?}", scores);
    Ok(())
}

async fn build_graph_from_scratch() -> Result<TflGraph> {
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
    client_options.app_name = Some("TravelTime".to_string());
    let mongo_client = mongodb::Client::with_options(client_options)?;
    let connection_repo = MongoRepository::<DirectConnection>::new(&mongo_client);
    let station_repo = MongoRepository::<StopPoint>::new(&mongo_client);
    let mut graph = TflGraph::new();
    graph
        .build_from_repos(&connection_repo, &station_repo)
        .await?;
    graph.add_walking_edges();
    Ok(graph)
}

struct LoadOptions {
    load_stops: bool,
    load_routes: bool,
    load_segments: bool,
    load_timetables: bool,
    fix_timetables: bool,
    fix_stoppoints: bool,
    load_national_rail_data: bool,
    load_national_rail: bool,
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
        println!("Loading timetables.");
        loader.load_timetables().await?;
        println!("Loaded timetables.");
    }

    if options.fix_timetables {
        println!("Fixing timetables.");
        DataFixer::fix_direct_connection_repo(&mongo_client).await?;
        println!("Done fixing timetables.");
    }

    if options.fix_stoppoints {
        println!("Fixing stop points.");
        DataFixer::fix_stop_point_repo(&mongo_client).await?;
        println!("Done fixing stop points.");
    }

    if options.load_national_rail_data {
        println!("Loading national rail timetables from S3.");
        NationalRailS3::get_timetable_data().await.unwrap();
        println!("Done loading national rail timetables from S3.");
    }

    if options.load_national_rail {
        println!("Loading timetables.");
        let timetable = TimetableLoader::new(&mongo_client);
        timetable.load_timetable("./data/timetable.xml").await?;
        println!("Loaded timetables.");
    }

    let _result = build_graph().await?;

    Ok(())
}
