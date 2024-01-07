use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse,
};

use shared::models::{CreateRoutine, Routine};
use uuid::Uuid;

use crate::routines_repository::RoutinesRepository;

pub fn service<R: RoutinesRepository>(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/v1/routines")
            // get all routines
            .route("", web::get().to(get_all::<R>))
            // get by id
            // .route("/{routine_id}", web::get().to(get::<R>))
            // post new routine
            .route("", web::post().to(post::<R>)),
        // update routine
        // .route("", web::put().to(put::<R>))
        // delete routine
        // .route("/{routine_id}", web::delete().to(delete::<R>)),
    );
}

async fn get_all<R: RoutinesRepository>(repo: web::Data<R>) -> HttpResponse {
    match repo.get_routines().await {
        Ok(routines) => HttpResponse::Ok().json(routines),
        Err(e) => HttpResponse::NotFound().body(format!("Internal server error: {:?}", e)),
    }
}

// async fn get<R: RoutineRepository>(routine_id: web::Path<Uuid>, repo: web::Data<R>) -> HttpResponse {
//     match repo.get_routine(&routine_id).await {
//         Ok(routine) => HttpResponse::Ok().json(routine),
//         Err(_) => HttpResponse::NotFound().body("Not found"),
//     }

async fn post<R: RoutinesRepository>(
    create_routine: web::Json<CreateRoutine>,
    repo: web::Data<R>,
) -> HttpResponse {
    match repo.create_routine(&create_routine).await {
        Ok(routine) => HttpResponse::Ok().json(routine),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Internal server error: {:?}", e))
        }
    }
}
