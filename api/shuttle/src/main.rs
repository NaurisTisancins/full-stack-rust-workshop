use actix_files::NamedFile;
use actix_web::{
    get,
    web::{self, ServiceConfig},
    Responder,
};
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::CustomError;
use sqlx::Executor;

#[get("/")]
async fn index() -> impl Responder {
    NamedFile::open_async("../static/index.html").await
}

#[shuttle_runtime::main]
async fn actix_web(
    #[shuttle_shared_db::Postgres()] pool: sqlx::PgPool,
    // #[shuttle_static_folder::StaticFolder(folder = "static")] static_folder: PathBuf,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    // initialize the database if not already initialized

    pool.execute(include_str!("../../db/schema.sql"))
        .await
        .map_err(CustomError::new)?;

    let routines_repository = api_lib::routines_repository::PostgresRoutinesRepository::new(pool);
    let routines_repository = actix_web::web::Data::new(routines_repository);

    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(
            web::scope("/api")
                .app_data(routines_repository)
                .configure(api_lib::health::service)
                .configure(
                    api_lib::routines::service::<
                        api_lib::routines_repository::PostgresRoutinesRepository,
                    >,
                ),
        );
    };

    Ok(config.into())
}
