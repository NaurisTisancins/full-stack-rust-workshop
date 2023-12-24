use actix_web::{get, web::ServiceConfig};
use shuttle_actix_web::ShuttleActixWeb;

// This function is an Actix Web handler that responds with the string "Hello World!"
//when accessed.
#[get("/")]
async fn hello_world() -> &'static str {
    "Hello World!"
}

#[get("/users")]
async fn get_users() -> &'static str {
    "USERS!"
}

// It is used to define the main function for the Shuttle runtime. Shuttle is a library
//for building concurrent, distributed, and parallel programs in Rust.
#[shuttle_runtime::main]
async fn actix_web(
    #[shuttle_shared_db::Postgres()] pool: sqlx::PgPool,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(hello_world);
    };

    //Finally, the closure config is converted into a ShuttleActixWeb type using the
    //into() method, and the result is wrapped in an Ok
    Ok(config.into())
}
