use serde::{Deserialize, Serialize};

use crate::db::mongo_doc::MongoDoc;

use super::stops_response::TransportMode;

pub type LinesResult = Vec<Line>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Line {
    #[serde(rename = "_id", alias = "id")]
    pub id: String,
    pub name: String,
    #[serde(rename = "modeName")]
    pub mode_name: TransportMode,
    #[serde(rename = "routeSections")]
    pub route_sections: Vec<RouteEndpoints>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RouteEndpoints {
    #[serde(rename = "_id", alias = "id")]
    pub id: Option<String>,
    pub name: String,
    pub direction: Direction,
    pub origination_name: String,
    pub destination_name: String,
    pub originator: String,
    pub destination: String,
    pub line_ids: Option<Vec<String>>,
}

impl MongoDoc for RouteEndpoints {
    fn database_name() -> &'static str {
        "tfl"
    }

    fn collection_name() -> &'static str {
        "routeEndPoints"
    }

    fn id(&self) -> String {
        let mut id = self.originator.clone();
        id.push_str(&self.destination);
        id
    }

    fn set_id(&mut self) {
        self.id = Some(self.id());
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Direction {
    Outbound,
    Inbound,
}
