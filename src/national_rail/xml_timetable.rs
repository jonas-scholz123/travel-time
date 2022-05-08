use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TimetableDoc {
    #[serde(rename = "Journey")]
    pub journeys: Vec<Journey>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Journey {
    #[serde(rename = "$value")]
    pub stops: Vec<StopType>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum StopType {
    #[serde(rename = "OR")]
    Origin {
        #[serde(rename = "tpl")]
        tiploc: String,
        #[serde(rename = "wtd")]
        departure: String,
    },
    #[serde(rename = "IP")]
    Intermediate {
        #[serde(rename = "tpl")]
        tiploc: String,
        #[serde(rename = "wta")]
        arrival: String,
        #[serde(rename = "wtd")]
        departure: String,
    },
    #[serde(rename = "DT")]
    Destination {
        #[serde(rename = "tpl")]
        tiploc: String,
        #[serde(rename = "wta")]
        arrival: String,
    },
    // Don't need these.
    #[serde(rename = "PP")]
    PassThrough,
    #[serde(rename = "OPOR")]
    OptionalOrigin,
    #[serde(rename = "OPIP")]
    OptionalIntermediate,
    #[serde(rename = "OPPP")]
    OptionalPassThrough,
    #[serde(rename = "OPDT")]
    OptionalDestination,
    #[serde(rename = "cancelReason")]
    CancelReason,
}
