use std::collections::{HashMap, HashSet};

use anyhow::Result;
use ball_tree::BallTree;
use futures::{stream, StreamExt, TryStreamExt};
use mongodb::Client;

use crate::{
    graph::{connection::Connection, node_pair::NodePair},
    tfl::model::{direct_connection::DirectConnection, stops_response::StopPoint},
};

use super::{mongo_doc::MongoDoc, mongo_repo::MongoRepository};

pub struct GraphLoader;

impl GraphLoader {
    pub async fn load_graph_data(client: &Client) -> Result<()> {
        let connection_repo: MongoRepository<DirectConnection> = MongoRepository::new(client);
        let stop_repo: MongoRepository<StopPoint> = MongoRepository::new(client);

        let cursor = connection_repo.get_all().await?;
        let edges = cursor.try_collect::<Vec<_>>().await?;

        let mut stop_ids = HashSet::new();
        edges.iter().for_each(|e| {
            stop_ids.insert(&e.origin);
            stop_ids.insert(&e.destination);
        });

        let stop_point_map = stream::iter(stop_ids)
            .map(|stop_id| stop_repo.get_by_id(stop_id))
            .buffer_unordered(30)
            //.try_collect::<Vec<_>>()
            .map(|stop| stop.unwrap().unwrap())
            .map(|stop| (stop.id.clone(), stop))
            .collect::<HashMap<_, _>>()
            .await;

        //while let Some(edge) = cursor.try_next().await? {
        let tfl_pairs: Vec<_> = edges
            .iter()
            .map(|edge| {
                let from = stop_point_map.get(&edge.origin).unwrap();

                let to = stop_point_map.get(&edge.destination).unwrap();
                let connection = Connection::from_direct_connection(edge);
                let mut np = NodePair {
                    from: from.into(),
                    to: to.into(),
                    edge: connection,
                };
                np
            })
            .collect();

        let nodes: HashMap<_, _> = tfl_pairs
            .iter()
            .flat_map(|p| vec![p.from.clone(), p.to.clone()])
            .map(|s| (s.id.clone(), s))
            .collect();
        let nodes: Vec<_> = nodes.values().collect();

        let locations: Vec<_> = nodes.iter().map(|l| l.location.clone()).collect();

        let ball_tree = BallTree::new(locations, nodes.to_vec());
        //let q = Query::nn_within(&mut self, point, max_radius)

        let walking_pairs: Vec<_> = nodes
            .into_iter()
            .flat_map(|node| {
                ball_tree
                    .query()
                    .nn_within(&node.location, 1000.)
                    .map(|(_, dist, &close_node)| {
                        let con = Connection::from_dist(dist);
                        let mut np = NodePair {
                            from: node.clone(),
                            to: close_node.clone(),
                            edge: con,
                        };
                        np.set_id();
                        np
                    })
                    .collect::<Vec<_>>()
            })
            .collect();

        let pair_repo: MongoRepository<NodePair> = MongoRepository::new(client);
        pair_repo.collection.drop(None).await?;
        pair_repo.insert_many(tfl_pairs).await?;
        pair_repo.insert_many(walking_pairs).await?;

        Ok(())
    }
}
