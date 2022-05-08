use std::{
    collections::hash_map::Entry::{Occupied, Vacant},
    collections::{BinaryHeap, HashMap},
    fs,
};

use crate::{
    db::mongo_repo::MongoRepository,
    tfl::model::{direct_connection::DirectConnection, stops_response::StopPoint},
};
use anyhow::Result;
use ball_tree::BallTree;
use chrono::NaiveTime;
use futures::TryStreamExt;
use petgraph::{
    graph::{EdgeReference, NodeIndex},
    visit::{EdgeRef, IntoNodeReferences, VisitMap, Visitable},
    Graph,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use super::{connection::Connection, min_scored::MinScored, path::Path, station::Station};

#[derive(Default, Serialize, Deserialize)]
pub struct TflGraph {
    graph: Graph<Station, Connection>,
    station_id_to_node: HashMap<String, NodeIndex>,
}

impl TflGraph {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn build_from_repos(
        &mut self,
        connection_repo: &MongoRepository<DirectConnection>,
        stop_repo: &MongoRepository<StopPoint>,
    ) -> Result<()> {
        let mut cursor = connection_repo.get_all().await?;

        while let Some(edge) = cursor.try_next().await? {
            let from_sp = stop_repo.get_by_id(edge.origin.clone()).await?;
            let to_sp = stop_repo.get_by_id(edge.destination.clone()).await?;
            let from_idx = TflGraph::get_or_insert_node_idx(
                &mut self.graph,
                &mut self.station_id_to_node,
                &from_sp.unwrap(),
            );
            let to_idx = TflGraph::get_or_insert_node_idx(
                &mut self.graph,
                &mut self.station_id_to_node,
                &to_sp.unwrap(),
            );

            let connection = Connection::from_direct_connection(&edge);

            self.graph.add_edge(from_idx, to_idx, connection);
        }

        Ok(())
    }

    pub fn add_walking_edges(&mut self) {
        let locations: Vec<_> = self
            .graph
            .node_weights()
            .map(|n| n.location.clone())
            .collect();
        let nodes: Vec<_> = self.graph.node_weights().collect();

        let ball_tree = BallTree::new(locations, nodes);
        //let q = Query::nn_within(&mut self, point, max_radius)
        let walking_connections = self
            .graph
            .node_references()
            .flat_map(|(idx, station)| {
                ball_tree
                    .query()
                    .nn_within(&station.location, 1000.)
                    .map(|(_, dist, &close_station)| {
                        let close_idx = self.station_id_to_node.get(&close_station.id).unwrap();
                        (idx, *close_idx, Connection::from_dist(dist))
                    })
                    .filter(|(idx1, idx2, _)| idx1 != idx2)
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
            self.graph.add_edge(idx, close_idx, con);
        }
    }

    pub async fn from_cache<S: Into<String>>(path: S) -> Result<Self> {
        //let graph = fs::read_to_string("/etc/hosts").expect("Unable to read file");
        let self_string = fs::read_to_string(path.into())?;
        Ok(serde_json::from_str(&self_string)?)
    }

    pub async fn cache<S: Into<String>>(&self, path: S) -> Result<()> {
        let graph_string = json!(self).to_string();
        fs::write(path.into(), graph_string)?;
        Ok(())
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
        let start_idx = *self.station_id_to_node.get(&start).unwrap();
        // All nodes should be in here.
        visit_next.push(MinScored(start_score, start_idx));

        while let Some(MinScored(node_score, node_idx)) = visit_next.pop() {
            if visited.is_visited(&node_idx) {
                continue;
            }
            let edges: Vec<EdgeReference<Connection>> = self.graph.edges(node_idx).collect();
            /*
                        println!(
                            "{:?} corresponding to station id {} has {} edges",
                            node_idx,
                            self.graph.node_weight(node_idx).unwrap().id,
                            edges.len()
                        );
            */
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

                if next == start_idx {
                    println!(
                        "CCC: start: {:?}, next: {:?}, current: {:?}",
                        start_idx, next, node_idx
                    );
                }

                match scores.entry(next) {
                    Occupied(ent) => {
                        let existing_score = *ent.get();
                        if next_score < existing_score {
                            *ent.into_mut() = next_score;
                            parents.insert(next, node_idx);
                            if next == start_idx {
                                println!(
                                    "AAA: start: {:?}, next: {:?}, current: {:?}",
                                    start_idx, next, node_idx
                                );
                            }
                        } else {
                            next_score = existing_score;
                        }
                    }
                    Vacant(ent) => {
                        ent.insert(next_score);
                        parents.insert(next, node_idx);
                        if next == start_idx {
                            println!(
                                "BBB: start: {:?}, next: {:?}, current: {:?}",
                                start_idx, next, node_idx
                            );
                        }
                    }
                }
                visit_next.push(MinScored(next_score, next));
            }
            visited.visit(node_idx);
        }
        println!("compiling scores");
        println!(
            "Starting score parent is some: {:#?}",
            parents.get(&start_idx)
        );
        scores
            .into_iter()
            .take(50)
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
            path.push(*parent);
        }
        path
    }
}
