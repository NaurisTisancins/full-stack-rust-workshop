use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse,
};

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
        .append_header(("version", "0.0.1"))
        .finish()
}
