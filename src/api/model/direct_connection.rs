use chrono::NaiveTime;
use serde::{Deserialize, Serialize};

use crate::db::mongo_doc::MongoDoc;

#[derive(Debug, Serialize, Deserialize)]
pub struct DirectConnection {
    #[serde(rename = "_id")]
    pub id: Option<String>,
    pub origin: String,
    pub destination: String,
    pub duration_minutes: f64,
    pub departure_times: Vec<NaiveTime>,
}

impl MongoDoc for DirectConnection {
    fn database_name() -> &'static str {
        "tfl"
    }

    fn collection_name() -> &'static str {
        "direct_connection"
    }

    fn id(&self) -> String {
        format!("{}-{}", self.origin, self.destination).to_string()
    }

    fn set_id(&mut self) {
        self.id = Some(self.id());
    }
}
