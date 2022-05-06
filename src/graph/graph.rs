use std::{
    collections::hash_map::Entry::{Occupied, Vacant},
    collections::{BinaryHeap, HashMap},
};

use crate::{
    api::model::{direct_connection::DirectConnection, stops_response::StopPoint},
    db::mongo_repo::MongoRepository,
};
use anyhow::Result;
use ball_tree::BallTree;
use chrono::NaiveTime;
use futures::TryStreamExt;
use petgraph::{
    graph::{EdgeReference, NodeIndex},
    visit::{EdgeRef, IntoEdges, IntoNodeReferences, VisitMap, Visitable},
    Graph,
};

use super::{connection::Connection, min_scored::MinScored, path::Path, station::Station};

pub struct TflGraph {
    graph: Graph<Station, Connection>,
    station_id_to_node: HashMap<String, NodeIndex>,
}

impl TflGraph {
    pub async fn from_cache() -> Result<Self> {
        todo!()
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

    pub async fn from_repos(
        connection_repo: &MongoRepository<DirectConnection>,
        stop_repo: &MongoRepository<StopPoint>,
    ) -> Result<Self> {
        let mut graph = petgraph::Graph::new();
        let mut cursor = connection_repo.get_all().await?;
        let mut station_id_to_node = HashMap::new();

        while let Some(edge) = cursor.try_next().await? {
            let from_sp = stop_repo.get_by_id(edge.origin.clone()).await?;
            let to_sp = stop_repo.get_by_id(edge.destination.clone()).await?;

            if from_sp.is_none() {
                println!("Error: Station not found: {}", edge.origin);
                continue;
            }

            if to_sp.is_none() {
                println!("Error: Station not found: {}", edge.destination);
                continue;
            }

            let from_idx = TflGraph::get_or_insert_node_idx(
                &mut graph,
                &mut station_id_to_node,
                &from_sp.unwrap(),
            );
            let to_idx = TflGraph::get_or_insert_node_idx(
                &mut graph,
                &mut station_id_to_node,
                &to_sp.unwrap(),
            );

            let connection = Connection::from_direct_connection(&edge);

            graph.add_edge(from_idx, to_idx, connection);
        }

        let locations: Vec<_> = graph.node_weights().map(|n| n.location.clone()).collect();
        let nodes: Vec<_> = graph.node_weights().collect();

        let ball_tree = BallTree::new(locations, nodes);
        //let q = Query::nn_within(&mut self, point, max_radius)
        let walking_connections = graph
            .node_references()
            .flat_map(|(idx, station)| {
                ball_tree
                    .query()
                    .nn_within(&station.location, 1000.)
                    .map(|(_, dist, &close_station)| {
                        let close_idx = station_id_to_node.get(&close_station.id).unwrap();
                        (idx, close_idx, Connection::from_dist(dist))
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        //let printies = walking_connections
        //    .iter()
        //    .map(|(n1, n2, con)| {
        //        (
        //            graph.node_weight(*n1).unwrap(),
        //            graph.node_weight(**n2).unwrap(),
        //            con.duration_minutes,
        //        )
        //    })
        //    .collect::<Vec<_>>();

        //println!("{:#?}", printies);

        for (idx, close_idx, con) in walking_connections {
            graph.add_edge(idx, *close_idx, con);
        }

        Ok(Self {
            graph,
            station_id_to_node,
        })
    }

    pub fn time_dependent_dijkstra(
        &self,
        start: String,
        start_time: NaiveTime,
    ) -> Vec<Path<Station>> {
        let mut visited = self.graph.visit_map();
        let mut scores = HashMap::new();
        let mut parents: HashMap<NodeIndex, NodeIndex> = HashMap::new();

        let mut visit_next = BinaryHeap::new();
        let start_score = (start_time - NaiveTime::from_hms(0, 0, 0)).num_minutes() as u16;
        // All nodes should be in here.
        visit_next.push(MinScored(
            start_score,
            *self.station_id_to_node.get(&start).unwrap(),
        ));

        while let Some(MinScored(node_score, node_idx)) = visit_next.pop() {
            if visited.is_visited(&node_idx) {
                continue;
            }
            let edges: Vec<EdgeReference<Connection>> = self.graph.edges(node_idx).collect();
            println!(
                "{:?} corresponding to station id {} has {} edges",
                node_idx,
                self.graph.node_weight(node_idx).unwrap().id,
                edges.len()
            );
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
                println!(
                    "time to depart: {}, travel time: {}, node score: {}",
                    time_to_depart, travel_time, node_score
                );

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
                    }
                }
                visit_next.push(MinScored(next_score, next));
            }
            visited.visit(node_idx);
        }
        scores
            .into_iter()
            .map(|(n_idx, score)| Path::<Station> {
                score: score - start_score,
                val: self.graph.node_weight(n_idx).unwrap().clone(),
                path: TflGraph::get_path(&parents, n_idx)
                    .iter()
                    .map(|idx| self.graph.node_weight(*idx).unwrap())
                    .collect(),
            })
            .collect()
    }

    fn get_path(parents: &HashMap<NodeIndex, NodeIndex>, child: NodeIndex) -> Vec<NodeIndex> {
        let mut path = vec![child];

        while let Some(parent) = parents.get(path.last().unwrap()) {
            path.push(*parent)
        }
        path
    }
}
