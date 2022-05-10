use serde::{Deserialize, Serialize};

use crate::db::mongo_doc::MongoDoc;

use super::{connection::Connection, station::Station};

#[derive(Serialize, Deserialize, Debug)]
pub struct NodePair {
    pub from: Station,
    pub to: Station,
    pub edge: Connection,
}

impl MongoDoc for NodePair {
    fn database_name() -> &'static str {
        "graph"
    }

    fn collection_name() -> &'static str {
        "nodePair"
    }

    fn id(&self) -> String {
        format!("{}{}", self.from.id, self.to.id)
    }
}
