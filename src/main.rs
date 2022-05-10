use anyhow::Result;
use chrono::NaiveTime;
use db::tfl_loader::Loader;
use graph::tfl_graph::TflGraph;
use mongodb::options::ClientOptions;
use tfl::client::TFLClient;
use tokio::time::Instant;

use crate::{
    db::{data_fixer::DataFixer, graph_loader::GraphLoader},
    national_rail::{s3::NationalRailS3, timetable_loader::TimetableLoader},
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
        load_timetables: false,
        fix_timetables: false,
        fix_stoppoints: false,
        load_national_rail_data: false,
        load_national_rail: false,
        load_graph_data: true,
        build_graph: false,
        cache: false,
    });

    let printstr = match result.await {
        Ok(_) => "Load completed successfully.".into(),
        Err(e) => e.to_string(),
    };
    println!("{}", printstr);
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
    load_graph_data: bool,
    build_graph: bool,
    cache: bool,
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

    if options.load_graph_data {
        println!("Loading graph data.");
        GraphLoader::load_graph_data(&mongo_client).await?;
        println!("Loaded graph data.");
    }

    if options.build_graph {
        println!("Building graph");
        let path = "./cache/graph.json";
        let graph = match options.cache {
            true => match TflGraph::from_cache(path).await {
                Ok(graph) => graph,
                Err(e) => {
                    println!("Error while reading from cache, falling back: {}", e);
                    let g = TflGraph::new(mongo_client).await?;
                    g.cache(path).await?;
                    g
                }
            },
            false => TflGraph::new(mongo_client).await?,
        };

        println!("Computing dijkstra's algorithm.");
        let now = Instant::now();
        let _scores =
            graph.time_dependent_dijkstra("490004733C".into(), NaiveTime::from_hms(10, 0, 0));
        println!("Time for dijkstra's: {}ms", now.elapsed().as_millis());
        //println!("{:#?}", scores);
    }

    Ok(())
}
