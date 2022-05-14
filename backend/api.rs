use std::{env, time::Instant};

use anyhow::Result;
use chrono::NaiveTime;
use mongodb::options::ClientOptions;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{get, routes, serde::json::Json, State};
use rocket::{Request, Response};
use tokio::sync::RwLock;

pub struct Cors;

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

use crate::graph::{location::Location, path::Path, tfl_graph::TflGraph};

#[get("/traveltime/<loc_string>/<time_str>")]
pub async fn get_travel_time(
    loc_string: String,
    time_str: String,
    graph: &State<RwLock<TflGraph>>,
) -> Json<Vec<Path>> {
    let now = Instant::now();

    let start_time = NaiveTime::parse_from_str(&time_str, "%H:%M").unwrap();

    if let Some(coords) = Location::try_parse_loc(&loc_string) {
        return Json(graph.write().await.tt_from_location(coords, start_time));
    }

    let results = graph
        .read()
        .await
        .tt_from_stop_id(loc_string, start_time)
        .unwrap();

    println!(
        "travel time request completed in {}ms",
        now.elapsed().as_millis()
    );

    Json(results)
}

pub async fn rocket() -> Result<()> {
    let port = env::var("PORT")
        .unwrap_or_else(|_| "3001".to_string())
        .parse::<usize>()?;

    println!("PORT: {:#?}", port);

    let atlas_uri = env::var("MONGO_URI").unwrap();
    let mut atlas_opts = ClientOptions::parse(atlas_uri).await?;
    atlas_opts.app_name = Some("travel-time".to_string());
    let atlas_client = mongodb::Client::with_options(atlas_opts)?;

    println!("Building graph");
    let now = Instant::now();
    let graph = RwLock::new(TflGraph::new(atlas_client).await?);
    println!("Done building graph in {}ms", now.elapsed().as_millis());

    let config = rocket::Config::figment().merge(("port", port));

    let _rocket = rocket::custom(config)
        .mount("/", routes![get_travel_time])
        .manage(graph)
        .attach(Cors)
        .ignite()
        .await?
        .launch()
        .await?;

    Ok(())
}
