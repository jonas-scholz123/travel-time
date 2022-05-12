use serde::{Deserialize, Serialize};

use crate::tfl::endpoint::Endpoint;

use super::time_table_response::TimetableResult;

#[derive(Debug, Serialize, Deserialize)]
pub struct TimetableRequest {
    #[serde(skip)]
    line_name: String,
    #[serde(skip)]
    from_station: String,
    #[serde(skip)]
    to_station: String,
}
//https://api.tfl.gov.uk/
impl Endpoint for TimetableRequest {
    fn method(&self) -> reqwest::Method {
        reqwest::Method::GET
    }

    fn endpoint(&self) -> String {
        format!(
            "Line/{}/Timetable/{}/to/{}",
            self.line_name, self.from_station, self.to_station
        )
    }

    type Returns = TimetableResult;
}

impl TimetableRequest {
    pub fn new<L: Into<String>, S: Into<String>>(line: L, from: S, to: S) -> Self {
        TimetableRequest {
            line_name: line.into(),
            from_station: from.into(),
            to_station: to.into(),
        }
    }
}
