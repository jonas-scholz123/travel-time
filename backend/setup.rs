use std::{env, time::Instant};

use crate::{
    db::{atlas_loader::copy_collections, data_fixer::DataFixer, tfl_loader::Loader},
    graph::mongo_graph_builder::MongoGraphBuilder,
    national_rail::{s3::NationalRailS3, timetable_loader::TimetableLoader},
    tfl::client::TFLClient,
    SetupArgs,
};
use anyhow::Result;
use chrono::NaiveTime;
use mongodb::options::ClientOptions;

pub async fn load(options: SetupArgs) -> Result<()> {
    let tfl_uri = env::var("TFL_CLIENT_URI")?;
    let mut tfl_client = TFLClient::new(&tfl_uri).unwrap();
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
        let graph_builder = MongoGraphBuilder::from_client(mongo_client).await;
        let graph = graph_builder.build_graph().await?;
        println!("Done building graph in {}ms", now.elapsed().as_millis());

        println!("Computing dijkstra's algorithm.");
        let now = Instant::now();
        let _scores = graph.tt_from_stop_id("490004733C".into(), NaiveTime::from_hms(10, 0, 0));
        println!("Time for dijkstra's: {}ms", now.elapsed().as_millis());
        //println!("{:#?}", scores);
    }

    Ok(())
}
