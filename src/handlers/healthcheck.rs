use rocket::{
    response::status,
    serde::json::Json,
};
use crate::{
    configs::BASE_URL,
    utils::{RequestId, CustomResponseBody, CustomResponse},
};

#[get("/healthcheck", format="application/json")]
pub async fn get(request_id: &RequestId) -> status::Custom<Json<CustomResponseBody>> {
    info!("request id: {}", request_id.number);
    let path = format!("{}/{}/healthcheck", *BASE_URL, super::NAMESPACE);

    let response = CustomResponse::init_for(request_id, &path);

    status::Custom(response.status, Json(response.body))
}
