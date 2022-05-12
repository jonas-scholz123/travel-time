use clap::Parser;
mod api;
mod db;
mod graph;
mod national_rail;
mod setup;
mod tfl;
mod utils;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct SetupArgs {
    /// Should stop points be loaded from the TFL api?
    #[clap(short, long)]
    stoppoint_load: bool,
    /// Should routes be loaded from the TFL api?
    #[clap(short, long)]
    routes_load: bool,
    /// Should route segments (i.e. between individual stops)
    /// be loaded from the TFL api?
    #[clap(short, long)]
    segment_load: bool,
    /// Should timetables (departure times) be loaded from the TFL api?
    #[clap(short, long)]
    timetable_load: bool,
    /// Should existing timetable data in Mongo be fixed?
    /// This involves sorting the departure times.
    #[clap(long)]
    fix_timetables: bool,
    /// Should existing stop point data in Mongo be fixed?
    /// This involves adding a (guessed) TIPLOC ID to every entry.
    #[clap(long)]
    fix_stoppoints: bool,
    /// Should we download the zip file containing national rail
    /// timetables from S3?
    #[clap(long)]
    load_national_rail_data: bool,
    /// Should national rail timetable data be inserted into Mongo?
    #[clap(short, long)]
    load_national_rail: bool,
    /// Should the TFLGraph be constructed locally?
    #[clap(long)]
    build_graph: bool,
    /// Should data be copied from the local MongoDB to
    /// the hosted one?
    #[clap(short, long)]
    copy_to_atlas: bool,
}

#[tokio::main]
async fn main() {
    let options = SetupArgs::parse();
    let result = crate::setup::load(options);
    let printstr = match result.await {
        Ok(_) => "Setup completed successfully.".into(),
        Err(e) => e.to_string(),
    };
    println!("{}", printstr);

    match crate::api::rocket().await {
        Ok(_) => println!("Terminated Successfully."),
        Err(e) => println!("{}", e),
    };
}
