use std::{
    collections::hash_map::Entry::{Occupied, Vacant},
    collections::{BinaryHeap, HashMap},
    hash::Hash,
};

use crate::{
    api::model::{direct_connection::DirectConnection, stops_response::StopPoint},
    db::mongo_repo::MongoRepository,
};
use anyhow::Result;
use chrono::NaiveTime;
use futures::{stream::Concat, StreamExt, TryStreamExt};
use geoutils::Location;
use petgraph::{
    algo::Measure,
    graph::{DefaultIx, EdgeReference, NodeIndex},
    visit::{EdgeRef, IntoEdges, VisitMap, Visitable},
    Graph,
};

use super::min_scored::MinScored;

#[derive(Clone, Debug)]
pub struct Station {
    id: String,
    location: Location,
    name: String,
    visited: bool,
}

#[derive(Debug)]
pub struct Scored<T> {
    score: u16,
    val: T,
}

impl Station {
    pub fn from_stop_point(sp: &StopPoint) -> Self {
        Self {
            id: sp.id.clone(),
            location: Location::new(sp.lat, sp.lon),
            name: sp.common_name.clone(),
            visited: false,
        }
    }
}

#[derive(Debug)]
pub struct Connection {
    from: String,
    to: String,
    duration_minutes: u16,
    departure_times: [u16; 24 * 60],
}

impl Connection {
    pub fn from_direct_connection(con: DirectConnection) -> Self {
        let mut departure_times_arr = [0; 24 * 60];
        let midnight = NaiveTime::from_hms(0, 0, 0);
        for depart_time in &con.departure_times {
            let idx = (*depart_time - midnight).num_minutes() as usize;
            departure_times_arr[idx] = 1;
        }

        Self {
            from: con.origin,
            to: con.destination,
            duration_minutes: con.duration_minutes as u16,
            departure_times: departure_times_arr,
        }
    }

    pub fn get_minutes_to_departure(&self, minutes_since_midnight: usize) -> u16 {
        self.departure_times[minutes_since_midnight]
    }
}

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
        //let mut station_id_to_station = HashMap::new();

        let connections = cursor.try_collect::<Vec<DirectConnection>>().await?;

        for edge in connections {
            let from_sp = stop_repo.get_by_id(edge.origin.clone()).await?;
            let to_sp = stop_repo.get_by_id(edge.destination.clone()).await?;

            if let None = from_sp {
                println!("Error: Station not found: {}", edge.origin);
                continue;
            }

            if let None = to_sp {
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

            let connection = Connection::from_direct_connection(edge);

            graph.add_edge(from_idx, to_idx, connection);
            let edges = graph.edges(from_idx).collect::<Vec<_>>();
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
    ) -> Vec<Scored<Station>> {
        let mut visited = self.graph.visit_map();
        let mut scores = HashMap::new();
        let nr_total_nodes = visited.len();

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
                let next_score = time_to_depart + travel_time + node_score;

                match scores.entry(next) {
                    Occupied(ent) => {
                        if next_score < *ent.get() {
                            *ent.into_mut() = next_score;
                            visit_next.push(MinScored(next_score, next));
                        }
                    }
                    Vacant(ent) => {
                        ent.insert(next_score);
                        visit_next.push(MinScored(next_score, next));
                    }
                }
            }
            visited.visit(node_idx);
        }
        scores
            .into_iter()
            .map(|(n_idx, score)| Scored::<Station> {
                score,
                val: self.graph.node_weight(n_idx).unwrap().clone(),
            })
            .collect()
    }

    pub fn dijkstra<G, F, K>(
        graph: G,
        start: G::NodeId,
        goal: Option<G::NodeId>,
        mut edge_cost: F,
    ) -> HashMap<G::NodeId, K>
    where
        G: IntoEdges + Visitable,
        G::NodeId: Eq + Hash,
        F: FnMut(G::EdgeRef) -> K,
        K: Measure + Copy,
    {
        let mut visited = graph.visit_map();
        let mut scores = HashMap::new();
        //let mut predecessor = HashMap::new();
        let mut visit_next = BinaryHeap::new();
        let zero_score = K::default();
        scores.insert(start, zero_score);
        visit_next.push(MinScored(zero_score, start));

        while let Some(MinScored(node_score, node)) = visit_next.pop() {
            if visited.is_visited(&node) {
                continue;
            }
            if goal.as_ref() == Some(&node) {
                break;
            }

            let edges = graph.edges(node);
            for edge in edges {
                let next = edge.target();
                if visited.is_visited(&next) {
                    continue;
                }
                let next_score = node_score + edge_cost(edge);
                match scores.entry(next) {
                    Occupied(ent) => {
                        if next_score < *ent.get() {
                            *ent.into_mut() = next_score;
                            visit_next.push(MinScored(next_score, next));
                            //predecessor.insert(next.clone(), node.clone());
                        }
                    }
                    Vacant(ent) => {
                        ent.insert(next_score);
                        visit_next.push(MinScored(next_score, next));
                        //predecessor.insert(next.clone(), node.clone());
                    }
                }
            }
            visited.visit(node);
        }
        scores
    }

    // TODO:
    // - Make a priority queue of edges, where the priority
    // is given by the time until next departure + travel time.
    // Only add nodes that have not yet been visited.
    // - Tick() -> advance global time until next departure + travel time.
    // - Visit all nodes that are reachable within this tick.
    // - Tick again.
    // Have scores: HashMap<NodeIndex, u16>, where a given node index
}
