use actix_web::web::ServiceConfig;
use api_lib::health::{hello_world, version};
use shuttle_actix_web::ShuttleActixWeb;

// It is used to define the main function for the Shuttle runtime. Shuttle is a library
//for building concurrent, distributed, and parallel programs in Rust.
#[shuttle_runtime::main]
async fn actix_web(
    #[shuttle_shared_db::Postgres()] pool: sqlx::PgPool,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let pool = actix_web::web::Data::new(pool);

    let config = move |cfg: &mut ServiceConfig| {
        cfg.app_data(pool).service(version).service(hello_world);
    };

    Ok(config.into())
}
