use anyhow::Result;
use futures::TryStreamExt;
use mongodb::Client;

use crate::tfl::model::{direct_connection::DirectConnection, stops_response::StopPoint};

use super::mongo_repo::MongoRepository;

pub async fn copy_collections(from_client: &Client, to_client: &Client) -> Result<()> {
    let from: MongoRepository<DirectConnection> = MongoRepository::new(from_client);
    let to: MongoRepository<DirectConnection> = MongoRepository::new(to_client);
    to.collection.drop(None).await?;

    let all = from.get_all().await?.try_collect::<Vec<_>>().await?;
    to.collection.insert_many(all, None).await?;

    let from: MongoRepository<StopPoint> = MongoRepository::new(from_client);
    let to: MongoRepository<StopPoint> = MongoRepository::new(to_client);
    to.collection.drop(None).await?;

    let all = from.get_all().await?.try_collect::<Vec<_>>().await?;
    to.collection.insert_many(all, None).await?;

    Ok(())
}
