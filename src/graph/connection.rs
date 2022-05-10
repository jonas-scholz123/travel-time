use chrono::NaiveTime;
use serde::{Deserialize, Serialize};

use crate::{db::mongo_doc::MongoDoc, tfl::model::direct_connection::DirectConnection};
use serde_big_array::BigArray;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Connection {
    pub duration_minutes: u16,
    // An array where the index is the minute of the day
    // and the value is the number of minutes until the
    // next train arrives.
    #[serde(with = "BigArray")]
    pub departure_times: [u16; 24 * 60],
}

impl Connection {
    pub fn from_direct_connection(con: &DirectConnection) -> Self {
        let mut departure_times_arr = [0; 24 * 60];
        let midnight = NaiveTime::from_hms(0, 0, 0);
        let mut start = 0_usize;
        // Depart times are pre-sorted in Mongo.
        for depart_time in &con.departure_times {
            let end = (*depart_time - midnight).num_minutes() as usize;

            let mut mins_until_depart = end - start;
            departure_times_arr
                .iter_mut()
                .take(end)
                .skip(start)
                .for_each(|entry| {
                    *entry = mins_until_depart as u16;
                    mins_until_depart -= 1;
                });
            /*
            for i in start..end {
                departure_times_arr[i] = mins_until_depart as u16;
                mins_until_depart -= 1;
            }
            */
            start = end + 1;
        }

        Self {
            duration_minutes: con.duration_minutes as u16,
            departure_times: departure_times_arr,
        }
    }

    pub fn get_minutes_to_departure(&self, minutes_since_midnight: usize) -> u16 {
        self.departure_times[minutes_since_midnight]
    }

    pub fn from_dist(dist: f64) -> Self {
        Self {
            // 200m per minute.
            duration_minutes: (dist / 80.) as u16,
            departure_times: [0; 24 * 60],
        }
    }
}
