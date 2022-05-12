use anyhow::Result;
use chrono::NaiveTime;
use serde::{Deserialize, Serialize};

use crate::db::{mongo_doc::MongoDoc, mongo_repo::MongoRepository};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
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
        "directConnection"
    }

    fn id(&self) -> String {
        format!("{}-{}", self.origin, self.destination)
    }

    fn set_id(&mut self) {
        self.id = Some(self.id());
    }
}

impl DirectConnection {
    pub async fn mongo_insert(&mut self, repo: &MongoRepository<DirectConnection>) -> Result<()> {
        let existing = repo.get_by_id(self.id()).await?;
        match existing {
            Some(mut existing) => {
                existing.departure_times.append(&mut self.departure_times);
                existing.departure_times.sort();
                existing.departure_times.dedup();
                repo.insert_or_replace(&existing).await?;
            }
            None => {
                self.set_id();
                self.departure_times.sort();
                self.departure_times.dedup();
                repo.insert(self).await?
            }
        };

        Ok(())
    }
}
