use std::ops::{Deref, DerefMut};

use ball_tree::Point;

#[derive(Clone, Debug, PartialEq)]
pub struct Location(pub geoutils::Location);

impl Deref for Location {
    type Target = geoutils::Location;

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
        self.haversine_distance_to(other).meters()
    }

    fn move_towards(&self, other: &Self, d: f64) -> Self {
        let dist = self.distance(other);
        if dist == 0. {
            return Location(geoutils::Location::new(self.latitude(), self.longitude()));
        }

        let fraction = d / dist;
        let lat = fraction * (other.latitude() - self.latitude());
        let lon = fraction * (other.longitude() - self.longitude());
        Location(geoutils::Location::new(lat, lon))
    }
}
