use serde::{Deserialize, Serialize};

use crate::tfl::model::stops_response::StopPoint;

use super::location::Location;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Station {
    pub id: String,
    pub location: Location,
    pub name: String,
}

impl Station {
    pub fn from_stop_point(sp: &StopPoint) -> Self {
        Self {
            id: sp.id.clone(),
            location: Location(geo::point!((sp.lat, sp.lon))),
            name: sp.common_name.clone(),
        }
    }
}
