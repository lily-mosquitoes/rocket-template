use chrono::{DateTime, Utc};
use rocket::{
    http::Status,
    serde::Serialize,
};
use erased_serde::Serialize as ErasedSerialize;
use crate::utils::RequestId;

pub struct CustomResponse {
    pub status: Status,
    pub body: CustomResponseBody,
}

pub trait CustomResponseData {
    fn as_response(&self, path: &String) -> Box<dyn ErasedSerialize>;
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct CustomResponseBody {
    pub request_id: usize,
    pub request_path: String,
    pub timestamp: DateTime<Utc>,
    pub status: CustomResponseStatus,
    pub data: Option<Box<dyn ErasedSerialize>>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct CustomResponseStatus {
    pub code: u16,
    pub reason: String,
    pub message: Option<String>,
}

impl CustomResponse {
    pub fn init_for(id: &RequestId, path: &String) -> CustomResponse {
        CustomResponse {
            status: Status::Ok,
            body: CustomResponseBody {
                request_id: id.number,
                request_path: path.to_owned(),
                timestamp: Utc::now(),
                status: CustomResponseStatus {
                    code: Status::Ok.code,
                    reason: Status::Ok.to_string(),
                    message: None,
                },
                data: None,
            },
        }
    }

    pub fn update_with(&mut self, status: Status, message: Option<String>, data: Option<Box<dyn ErasedSerialize>>) {
        self.status = status;
        self.body.status.code = status.code;
        self.body.status.reason = status.to_string();
        self.body.status.message = message;
        self.body.data = data;
    }
}
