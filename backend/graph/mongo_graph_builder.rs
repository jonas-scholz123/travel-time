use std::{collections::HashSet, env};

use anyhow::Result;
use futures::TryStreamExt;
use mongodb::{bson::doc, options::ClientOptions};

use crate::{
    db::mongo_repo::MongoRepository,
    tfl::model::{direct_connection::DirectConnection, stops_response::StopPoint},
};

use super::tfl_graph::TflGraph;

pub struct MongoGraphBuilder {
    connection_repo: MongoRepository<DirectConnection>,
    stop_repo: MongoRepository<StopPoint>,
}

impl MongoGraphBuilder {
    pub async fn from_client(client: mongodb::Client) -> Self {
        Self {
            connection_repo: MongoRepository::new(&client),
            stop_repo: MongoRepository::new(&client),
        }
    }

    pub async fn from_env_var() -> Result<Self> {
        let mongo_uri = env::var("MONGO_URI").unwrap();
        let mut client_options = ClientOptions::parse(mongo_uri).await?;
        client_options.app_name = Some("travel-time".to_string());
        let mongo_client = mongodb::Client::with_options(client_options)?;

        Ok(Self::from_client(mongo_client).await)
    }

    pub async fn build_graph(&self) -> Result<TflGraph> {
        let connections = self.get_all_connections().await?;

        let stop_ids: Vec<_> = connections
            .iter()
            .flat_map(|e| vec![&e.origin, &e.destination])
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();

        let stop_points = self.get_all_stop_points(stop_ids).await?;

        let mut graph = TflGraph::default();
        graph.add_stations(connections, stop_points)?;
        graph.add_walking_edges();

        Ok(graph)
    }

    async fn get_all_connections(&self) -> Result<Vec<DirectConnection>> {
        let cursor = self.connection_repo.get_all().await?;
        return Ok(cursor.try_collect::<Vec<_>>().await?);
    }

    async fn get_all_stop_points(&self, stop_ids: Vec<&String>) -> Result<Vec<StopPoint>> {
        let stop_ids_vec = stop_ids.iter().collect::<Vec<_>>();
        let filter = doc! {"_id": {"$in": stop_ids_vec}};
        let cursor = self.stop_repo.collection.find(filter, None).await?;
        let stop_points = cursor.try_collect::<Vec<_>>().await?;

        Ok(stop_points)
    }
}
