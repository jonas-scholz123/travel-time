use std::{env, time::Instant};

use crate::graph::{location::Location, path::Path, tfl_graph::TflGraph};
use anyhow::Result;
use chrono::NaiveTime;
use mongodb::options::ClientOptions;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{get, routes, serde::json::Json, State};
use rocket::{Request, Response};
use tokio::sync::RwLock;

#[get("/")]
pub fn wake_up() -> &'static str {
    "awake"
}

#[get("/traveltime/<loc_string>/<time_str>")]
pub async fn get_travel_time(
    loc_string: String,
    time_str: String,
    graph: &State<RwLock<TflGraph>>,
) -> Json<Vec<Path>> {
    let start_time = NaiveTime::parse_from_str(&time_str, "%H:%M").unwrap();

    println!("{}", loc_string);
    let locs: Result<Vec<_>> = loc_string
        .split('_')
        .map(|loc_str| {
            Location::try_parse_loc(loc_str)
                .ok_or_else(|| anyhow::anyhow!("Location string could not be parsed: {}", loc_str))
        })
        .collect();

    let result = match locs {
        Ok(coords_list) => graph
            .write()
            .await
            .tt_from_locations(coords_list, start_time),
        Err(e) => {
            println!("{}", e);
            vec![]
        }
    };

    Json(result)
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
        .mount("/", routes![wake_up, get_travel_time])
        .manage(graph)
        .attach(Cors)
        .ignite()
        .await?
        .launch()
        .await?;

    Ok(())
}

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
