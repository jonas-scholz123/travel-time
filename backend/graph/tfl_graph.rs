use std::{
    collections::hash_map::Entry::{Occupied, Vacant},
    collections::{BinaryHeap, HashMap, HashSet},
};

use crate::{
    db::mongo_repo::MongoRepository,
    tfl::model::{direct_connection::DirectConnection, stops_response::StopPoint},
};
use anyhow::{Context, Result};
use ball_tree::BallTree;
use chrono::NaiveTime;
use futures::TryStreamExt;
use mongodb::{bson::doc, Client};
use petgraph::{
    graph::{EdgeReference, NodeIndex},
    visit::{EdgeRef, IntoNodeReferences, VisitMap, Visitable},
    Graph,
};

use super::{
    connection::Connection, location::Location, min_scored::MinScored, path::Path, station::Station,
};

#[derive(Default)]
pub struct TflGraph {
    graph: Graph<Station, Connection>,
    ball_tree: Option<BallTree<Location, Station>>,
    station_id_to_node: HashMap<String, NodeIndex>,
}

impl<'a> TflGraph {
    pub async fn new(mongo_client: Client) -> Result<TflGraph> {
        let mut result = Self::default();
        result
            .add_stations(
                &MongoRepository::new(&mongo_client),
                &MongoRepository::new(&mongo_client),
            )
            .await?;
        result.add_walking_edges();
        Ok(result)
    }

    async fn add_stations(
        &mut self,
        connection_repo: &MongoRepository<DirectConnection>,
        stop_repo: &MongoRepository<StopPoint>,
    ) -> Result<()> {
        let cursor = connection_repo.get_all().await?;
        let edges = cursor.try_collect::<Vec<_>>().await?;

        let mut stop_ids = HashSet::new();
        edges.iter().for_each(|e| {
            stop_ids.insert(&e.origin);
            stop_ids.insert(&e.destination);
        });

        let stop_ids_vec = stop_ids.iter().collect::<Vec<_>>();
        let filter = doc! {"_id": {"$in": stop_ids_vec}};
        let cursor = stop_repo.collection.find(filter, None).await?;

        let stop_points = cursor.try_collect::<Vec<_>>().await?;

        let stop_point_map = stop_points
            .iter()
            .map(|s| (s.id.clone(), s))
            .collect::<HashMap<_, _>>();

        for edge in edges {
            let from_sp = stop_point_map.get(&edge.origin).unwrap();
            let to_sp = stop_point_map.get(&edge.destination).unwrap();
            let from_idx = TflGraph::get_or_insert_node_idx(
                &mut self.graph,
                &mut self.station_id_to_node,
                from_sp,
            );
            let to_idx = TflGraph::get_or_insert_node_idx(
                &mut self.graph,
                &mut self.station_id_to_node,
                to_sp,
            );

            let connection = Connection::from_direct_connection(&edge);

            self.graph.add_edge(from_idx, to_idx, connection);
        }

        Ok(())
    }

    fn get_walking_connections(
        &self,
        station: &Station,
        station_idx: NodeIndex,
    ) -> Vec<(NodeIndex, NodeIndex, Connection)> {
        self.ball_tree
            .as_ref()
            .unwrap()
            .query()
            .nn_within(&station.location, 1000.)
            .map(|(_, dist, close_station)| {
                let close_idx = self.station_id_to_node.get(&close_station.id).unwrap();
                (station_idx, *close_idx, Connection::from_dist(dist))
            })
            .filter(|(idx1, idx2, _)| idx1 != idx2)
            .collect::<Vec<_>>()
    }

    pub fn add_walking_edges(&mut self) {
        let locations: Vec<_> = self
            .graph
            .node_weights()
            .map(|n| n.location.clone())
            .collect();

        let nodes: Vec<_> = self.graph.node_weights().cloned().collect();

        self.ball_tree = Some(BallTree::new(locations, nodes));
        //let q = Query::nn_within(&mut self, point, max_radius)
        let walking_connections = self
            .graph
            .node_references()
            .flat_map(|(idx, station)| self.get_walking_connections(station, idx))
            .collect::<Vec<_>>();

        for (idx, close_idx, con) in walking_connections {
            self.graph.add_edge(idx, close_idx, con);
        }
    }

    fn get_or_insert_node_idx(
        graph: &mut Graph<Station, Connection>,
        map: &mut HashMap<String, NodeIndex>,
        stop_point: &StopPoint,
    ) -> NodeIndex {
        match map.entry(stop_point.id.clone()) {
            Occupied(entry) => {
                return *entry.get();
            }
            Vacant(entry) => {
                let station = Station::from_stop_point(stop_point);
                let idx = graph.add_node(station);
                entry.insert(idx);
                idx
            }
        }
    }

    pub fn tt_from_location(&mut self, start_loc: Location, start_time: NaiveTime) -> Vec<Path> {
        let start = Station {
            id: "".into(),
            location: start_loc,
            name: "".into(),
        };

        let start_idx = self.graph.add_node(start.clone());
        let connections = self.get_walking_connections(&start, start_idx);

        for (idx, close_idx, con) in connections {
            self.graph.add_edge(idx, close_idx, con);
        }

        let result = self.tt_from_start_idx(start_idx, start_time);

        // Remove the temporarily added start node.
        self.graph.remove_node(start_idx);

        result
    }

    pub fn tt_from_locations(
        &mut self,
        start_locs: Vec<Location>,
        start_time: NaiveTime,
    ) -> Vec<Path> {
        if start_locs.len() == 1 {
            return self
                .tt_from_location(Location(*start_locs.first().unwrap().clone()), start_time);
        }

        // Keep track of the longest time taken to a station.
        let mut longest_paths: HashMap<String, Path> = HashMap::new();

        for loc in start_locs {
            let paths = self.tt_from_location(loc, start_time);
            for path in paths {
                let key = path.destination.id.clone();

                match longest_paths.entry(key) {
                    Occupied(mut ent) => {
                        if (*ent.get()).minutes < path.minutes {
                            ent.insert(path);
                        }
                    }
                    Vacant(ent) => {
                        ent.insert(path);
                    }
                }
            }
        }

        longest_paths.into_iter().map(|(_, path)| path).collect()
    }

    pub fn tt_from_stop_id(&self, start: String, start_time: NaiveTime) -> Result<Vec<Path>> {
        let start_idx = *self
            .station_id_to_node
            .get(&start)
            .context("Invalid stop point ID")?;

        Ok(self.tt_from_start_idx(start_idx, start_time))
    }

    fn tt_from_start_idx(&self, start_idx: NodeIndex, start_time: NaiveTime) -> Vec<Path> {
        let mut visited = self.graph.visit_map();
        let mut scores = HashMap::new();
        let mut parents: HashMap<NodeIndex, NodeIndex> = HashMap::new();

        let mut visit_next = BinaryHeap::new();
        let start_score = (start_time - NaiveTime::from_hms(0, 0, 0)).num_minutes() as u16;
        // All nodes should be in here.
        visit_next.push(MinScored(start_score, start_idx));

        while let Some(MinScored(node_score, node_idx)) = visit_next.pop() {
            if visited.is_visited(&node_idx) {
                continue;
            }
            let edges: Vec<EdgeReference<Connection>> = self.graph.edges(node_idx).collect();
            for edge in edges {
                let next = edge.target();
                if visited.is_visited(&next) {
                    continue;
                }

                let edge_weight = edge.weight();
                let time_to_depart = edge_weight.get_minutes_to_departure(node_score as usize);
                let travel_time = edge_weight.duration_minutes;
                // Score is the number of minutes required to reach the node since the start of the journey.
                let mut next_score = time_to_depart + travel_time + node_score;

                match scores.entry(next) {
                    Occupied(ent) => {
                        let existing_score = *ent.get();
                        if next_score < existing_score {
                            *ent.into_mut() = next_score;
                            parents.insert(next, node_idx);
                        } else {
                            next_score = existing_score;
                        }
                    }
                    Vacant(ent) => {
                        ent.insert(next_score);
                        parents.insert(next, node_idx);
                    }
                }
                visit_next.push(MinScored(next_score, next));
            }
            visited.visit(node_idx);
        }
        scores
            .into_iter()
            .map(|(n_idx, score)| Path {
                minutes: score - start_score,
                destination: self.graph.node_weight(n_idx).unwrap().clone(),
                //path: TflGraph::get_path(&parents, n_idx)
                //    .iter()
                //    .map(|idx| self.graph.node_weight(*idx).unwrap().id.clone())
                //    .collect(),
                path: vec![],
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use geo::Point;
    use mongodb::options::ClientOptions;

    use super::*;

    #[tokio::test]
    async fn test_from_location() {
        let mut atlas_opts = ClientOptions::parse("mongodb://localhost:27017")
            .await
            .unwrap();
        atlas_opts.app_name = Some("travel-time".to_string());
        let atlas_client = mongodb::Client::with_options(atlas_opts).unwrap();
        let mut graph = TflGraph::new(atlas_client).await.unwrap();

        let loc = Location(Point::new(51.501105, -0.232320));
        let time = NaiveTime::from_hms(10, 0, 0);
        let results = graph.tt_from_location(loc, time);
        assert!(!results.is_empty());
    }
}
