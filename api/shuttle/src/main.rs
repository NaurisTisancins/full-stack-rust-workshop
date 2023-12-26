use actix_web::{get, web::ServiceConfig};
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::CustomError;
use sqlx::Executor;

// This function is an Actix Web handler that responds with the string "Hello World!"
//when accessed.
#[get("/version")]
async fn version(db: actix_web::web::Data<sqlx::PgPool>) -> String {
    tracing::info!("Getting version");
    let result: Result<String, sqlx::Error> = sqlx::query_scalar("SELECT version()")
        .fetch_one(db.as_ref())
        .await;

    match result {
        Ok(version) => format!("Hello World! {}", version),
        Err(e) => format!("Error: {:?}", e),
    }
}

// It is used to define the main function for the Shuttle runtime. Shuttle is a library
//for building concurrent, distributed, and parallel programs in Rust.
#[shuttle_runtime::main]
async fn actix_web(
    #[shuttle_shared_db::Postgres()] pool: sqlx::PgPool,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let pool = actix_web::web::Data::new(pool);

    let config = move |cfg: &mut ServiceConfig| {
        cfg.app_data(pool).service(version);
    };

    Ok(config.into())
}
