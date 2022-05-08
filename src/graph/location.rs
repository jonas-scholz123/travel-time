use std::ops::{Deref, DerefMut};

use ball_tree::Point;
use geo::prelude::HaversineDistance;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Location(pub geo::Point<f64>);

impl Deref for Location {
    type Target = geo::Point<f64>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Location {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Point for Location {
    fn distance(&self, other: &Self) -> f64 {
        self.haversine_distance(other)
    }

    fn move_towards(&self, other: &Self, d: f64) -> Self {
        let dist = self.distance(other);
        if dist == 0. {
            return Location(*self.clone());
        }

        let point = (self.0 + other.0) * d / dist;
        Location(point)
    }
}
