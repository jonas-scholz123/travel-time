use serde::{Deserialize, Serialize};

use super::station::Station;

#[derive(Debug, Serialize, Deserialize)]
pub struct Path {
    pub minutes: u16,
    pub destination: Station,
    pub path: Option<Vec<Station>>,
}
