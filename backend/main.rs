use std::env;

use anyhow::Result;
use chrono::NaiveTime;
use clap::Parser;
use db::tfl_loader::Loader;
use graph::tfl_graph::TflGraph;
use mongodb::options::ClientOptions;
use rocket::{serde::json::Json, State};
use tfl::client::TFLClient;
use tokio::time::Instant;

use crate::{
    db::{atlas_loader::copy_collections, data_fixer::DataFixer},
    graph::path::Path,
    national_rail::{s3::NationalRailS3, timetable_loader::TimetableLoader},
};

mod db;
pub mod graph;
pub mod national_rail;
mod tfl;
pub mod utils;

#[macro_use]
extern crate rocket;

#[get("/traveltime/<stop_id>/<time_str>")]
fn get_travel_time(stop_id: String, time_str: String, graph: &State<TflGraph>) -> Json<Vec<Path>> {
    let now = Instant::now();
    let time = NaiveTime::parse_from_str(&time_str, "%H:%M").unwrap();
    let results = graph.time_dependent_dijkstra(stop_id, time);
    println!(
        "travel time request completed in {}ms",
        now.elapsed().as_millis()
    );
    Json(results)
}

//#[launch]
async fn rocket() -> Result<()> {
    let port = env::var("PORT")
        .unwrap_or_else(|_| "5000".to_string())
        .parse::<usize>()?;

    println!("PORT: {:#?}", port);

    let atlas_uri = env::var("MONGO_URI").unwrap();
    let mut atlas_opts = ClientOptions::parse(atlas_uri).await?;
    atlas_opts.app_name = Some("travel-time".to_string());
    let atlas_client = mongodb::Client::with_options(atlas_opts)?;

    println!("Building graph");
    let now = Instant::now();
    let graph = TflGraph::new(atlas_client).await?;
    println!("Done building graph in {}ms", now.elapsed().as_millis());

    let config = rocket::Config::figment().merge(("port", port));

    let _rocket = rocket::custom(config)
        .mount("/", routes![get_travel_time])
        .manage(graph)
        .ignite()
        .await?
        .launch()
        .await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    let options = Args::parse();
    let result = load(options);
    let printstr = match result.await {
        Ok(_) => "Setup completed successfully.".into(),
        Err(e) => e.to_string(),
    };
    println!("{}", printstr);

    match rocket().await {
        Ok(_) => println!("Terminated Successfully."),
        Err(e) => println!("{}", e),
    };
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    // Should stop points be loaded from the TFL api?
    #[clap(short, long)]
    stoppoint_load: bool,
    // Should routes be loaded from the TFL api?
    #[clap(short, long)]
    routes_load: bool,
    // Should route segments (i.e. between individual stops)
    // be loaded from the TFL api?
    #[clap(short, long)]
    segment_load: bool,
    // Should timetables (departure times) be loaded from the TFL api?
    #[clap(short, long)]
    timetable_load: bool,
    // Should existing timetable data in Mongo be fixed?
    // This involves sorting the departure times.
    #[clap(long)]
    fix_timetables: bool,
    // Should existing stop point data in Mongo be fixed?
    // This involves adding a (guessed) TIPLOC ID to every entry.
    #[clap(long)]
    fix_stoppoints: bool,
    // Should we download the zip file containing national rail
    // timetables from S3?
    #[clap(long)]
    load_national_rail_data: bool,
    // Should national rail timetable data be inserted into Mongo?
    #[clap(short, long)]
    load_national_rail: bool,
    // Should the TFLGraph be constructed locally?
    #[clap(long)]
    build_graph: bool,
    // Should data be copied from the local MongoDB to
    // the hosted one?
    #[clap(short, long)]
    copy_to_atlas: bool,
}

async fn load(options: Args) -> Result<()> {
    let mut tfl_client = TFLClient::new("7fa56d767da04461a225dfe82d34ef51").unwrap();
    // Parse a connection string into an options struct.
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
    client_options.app_name = Some("TravelTime".to_string());

    // Get a handle to the deployment.
    let mongo_client = mongodb::Client::with_options(client_options)?;
    println!("Client OK.");

    let mut loader = Loader::new(&mut tfl_client, &mongo_client);

    if options.stoppoint_load {
        println!("Loading stops.");
        loader.load_stops().await?;
        println!("Loaded stops.");
    }

    if options.routes_load {
        println!("Loading routes.");
        loader.load_routes().await?;
        println!("Loaded routes.");
    }

    if options.segment_load {
        println!("Loading segments.");
        loader.load_segments().await?;
        println!("Loaded segments.");
    }

    if options.timetable_load {
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

    if options.copy_to_atlas {
        let atlas_uri = env::var("MONGO_URI")?;
        println!("{}", atlas_uri);
        let mut atlas_opts = ClientOptions::parse(atlas_uri).await?;
        atlas_opts.app_name = Some("travel-time".to_string());
        let atlas_client = mongodb::Client::with_options(atlas_opts)?;
        println!("Copying collections");
        copy_collections(&mongo_client, &atlas_client).await?;
        println!("Pasting collections");
    }

    if options.build_graph {
        println!("Building graph");
        let now = Instant::now();
        let graph = TflGraph::new(mongo_client).await?;
        println!("Done building graph in {}ms", now.elapsed().as_millis());

        println!("Computing dijkstra's algorithm.");
        let now = Instant::now();
        let _scores =
            graph.time_dependent_dijkstra("490004733C".into(), NaiveTime::from_hms(10, 0, 0));
        println!("Time for dijkstra's: {}ms", now.elapsed().as_millis());
        //println!("{:#?}", scores);
    }

    Ok(())
}
