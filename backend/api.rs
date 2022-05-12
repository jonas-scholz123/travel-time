use std::{env, time::Instant};

use anyhow::Result;
use chrono::NaiveTime;
use mongodb::options::ClientOptions;
use rocket::{get, routes, serde::json::Json, State};

use crate::graph::{path::Path, tfl_graph::TflGraph};

#[get("/traveltime/<stop_id>/<time_str>")]
pub fn get_travel_time(
    stop_id: String,
    time_str: String,
    graph: &State<TflGraph>,
) -> Json<Vec<Path>> {
    let now = Instant::now();
    let time = NaiveTime::parse_from_str(&time_str, "%H:%M").unwrap();
    let results = graph.time_dependent_dijkstra(stop_id, time);
    println!(
        "travel time request completed in {}ms",
        now.elapsed().as_millis()
    );
    Json(results)
}

pub async fn rocket() -> Result<()> {
    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
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
