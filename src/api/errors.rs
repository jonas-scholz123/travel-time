use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ServerResponse<T> {
    Success(T),
    BadRequest(TflBadRequest),
}

/*
   E.g.
   "timestampUtc": "2022-05-02T11:48:17.9860598Z",
   "exceptionType": "ApiArgumentException",
   "httpStatusCode": 400,
   "httpStatus": "BadRequest",
   "relativeUri": "/Line/Mode/national-rail,dlr,tube,bus,cable-car,coach,cycle,cycle-hire,dlr,elizabeth-line,asd",
   "message": "The following modes are not recognised: elizabeth-line, asd"
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TflBadRequest {
    pub timestamp_utc: Option<String>,
    pub exception_type: String,
    pub http_status_code: i32,
    pub message: String,
}
