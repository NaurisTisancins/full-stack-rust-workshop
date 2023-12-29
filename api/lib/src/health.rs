use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse,
};

pub const API_VERSION: &str = "v0.0.1";

pub fn service(cfg: &mut ServiceConfig) {
    cfg.route("/health", web::get().to(health));
}
//alternative when using #[get("/health")] macro
// pub fn service(cfg: &mut ServiceConfig) {
//     cfg.service(health);
// }

// #[get("/health")] // automatically adds pub keyword to the function
async fn health() -> HttpResponse {
    HttpResponse::Ok()
        .append_header(("version", API_VERSION))
        .finish()
}

#[cfg(test)]
mod tests {
    use actix_web::http::StatusCode;

    use super::*;

    #[actix_rt::test]
    async fn health_check_works() {
        let res = health().await;

        assert!(res.status().is_success());
        assert_eq!(res.status(), StatusCode::OK);

        let data = res.headers().get("version").and_then(|h| h.to_str().ok());

        assert_eq!(data, Some(API_VERSION));
    }
}
