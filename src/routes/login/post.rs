use actix_web::HttpResponse;
use actix_web::http::header::LOCATION;
use secrecy::Secret;

#[derive(serde::Deserialize)]
pub struct Formdata {
    username: String,
    password: Secret<String>,
}

pub async fn login() -> HttpResponse {
    HttpResponse::SeeOther()
        .insert_header((LOCATION, "/"))
        .finish()
}