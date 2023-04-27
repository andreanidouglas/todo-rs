use actix_web::{get, HttpResponse, Responder};

#[tracing::instrument(name = "GET: /health_check")]
#[get("/health_check")]
pub async fn health_check() -> impl Responder {
    tracing::info!("request GET: /health_check");
    HttpResponse::Ok().finish()
}
