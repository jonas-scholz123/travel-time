use std::collections::{HashMap, HashSet};

use anyhow::{Ok, Result};
use futures::TryStreamExt;
use mongodb::{bson::doc, Client, IndexModel};

use crate::tfl::model::{direct_connection::DirectConnection, stops_response::StopPoint};

use super::mongo_repo::MongoRepository;

pub struct DataFixer;

impl DataFixer {
    pub async fn fix_direct_connection_repo(client: &Client) -> Result<()> {
        let repo = MongoRepository::<DirectConnection>::new(client);
        let mut cursor = repo.get_all().await?;
        while let Some(con) = cursor.try_next().await? {
            let mut new_con = con.clone();
            new_con.departure_times.sort();
            new_con.departure_times.dedup();
            repo.insert_or_replace(&new_con).await?;
        }

        Ok(())
    }

    pub async fn fix_stop_point_repo(client: &Client) -> Result<()> {
        // Mark all stations that are in graph.
        let repo = MongoRepository::<StopPoint>::new(client);
        let index = IndexModel::builder().keys(doc! {"tiploc": 1}).build();
        repo.collection.create_index(index, None).await?;
        let mut cursor = repo.get_all().await?;
        while let Some(mut stop) = cursor.try_next().await? {
            stop.tiploc = Some(DataFixer::id_to_tiploc(&stop.id));
            repo.insert_or_replace(&stop).await?;
        }

        Ok(())
    }

    fn id_to_tiploc(id: &str) -> String {
        // No numbers allowed
        id.chars().filter(|c| c.is_alphabetic()).collect::<String>()
    }
}
