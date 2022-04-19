use rocket::{
    http::Status,
    response::status,
    serde::json::Json,
};
use crate::{
    configs::BASE_URL,
    utils::{RequestId, CustomResponseBody, CustomResponseData, CustomResponse},
};

#[get("/template_other", format="application/json")]
pub async fn get(request_id: &RequestId) -> status::Custom<Json<CustomResponseBody>> {
    info!("request id: {}", request_id.number);
    let path = format!("{}/{}/template_other", *BASE_URL, super::NAMESPACE);

    // example
    // let query = connection.run(move |c|
    //     repository::processes::get(&process_id, c))
    //     .await;
    let query: Result<crate::models::template::Template, anyhow::Error> = Ok(crate::models::template::Template {
        id: 69,
        name: "Lílian".to_string(),
        yes: true,
    });
    let (status, message, data) = match query {
        Ok(process) => {
            (Status::Ok, None, Some(process.as_response(&path)))
        },
        Err(error) => {
            error!("error: {:?}; path: {}", error, &path);
            (Status::InternalServerError, Some(error.to_string()), None)
        },
    };

    let mut response = CustomResponse::init_for(request_id, &path);
    response.update_with(status, message, data);

    status::Custom(response.status, Json(response.body))
}
