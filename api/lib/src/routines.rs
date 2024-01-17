use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse,
};

use shared::models::{
    CreateExercise, CreateRoutine, CreateTrainingDay, Exercise, Routine, TrainingDay,
};
use uuid::Uuid;

use crate::routines_repository::RoutinesRepository;

pub fn service<R: RoutinesRepository>(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/v1")
            // get all routines
            .route("/routines", web::get().to(get_all_routines::<R>))
            .route("/routines/active", web::get().to(get_active_routine::<R>))
            .route("/routines", web::post().to(create_routine::<R>))
            .route(
                "/routines/{routine_id}",
                web::delete().to(delete_routine::<R>),
            )
            .route(
                "/training_days", // post new training day
                web::post().to(create_training_day::<R>),
            )
            .route(
                "/training_days/{routine_id}",
                web::get().to(get_training_days::<R>),
            )
            .route("/exercises", web::post().to(create_exercise::<R>))
            .route("/exercises", web::get().to(get_exercises::<R>))
            .route(
                "/exercises/{exercise_id}/{day_id}",
                web::post().to(add_exercise_to_training_day::<R>),
            ),
    );
}
// ROUTINES
async fn get_all_routines<R: RoutinesRepository>(repo: web::Data<R>) -> HttpResponse {
    match repo.get_routines().await {
        Ok(routines) => HttpResponse::Ok().json(routines),
        Err(e) => HttpResponse::NotFound().body(format!("Internal server error: {:?}", e)),
    }
}

async fn get_active_routine<R: RoutinesRepository>(repo: web::Data<R>) -> HttpResponse {
    match repo.get_active_routine().await {
        Ok(routine) => HttpResponse::Ok().json(routine),
        Err(e) => HttpResponse::NotFound().body(format!("Internal server error: {:?}", e)),
    }
}

async fn create_routine<R: RoutinesRepository>(
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

async fn delete_routine<R: RoutinesRepository>(
    routine_id: web::Path<Uuid>,
    repo: web::Data<R>,
) -> HttpResponse {
    let routine_id = routine_id.into_inner();
    match repo.delete_routine(&routine_id).await {
        Ok(routine_id) => HttpResponse::Ok().json(routine_id),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Internal server error: {:?}", e))
        }
    }
}

// TRAINING DAYS
async fn get_training_days<R: RoutinesRepository>(
    path: web::Path<Uuid>,
    repo: web::Data<R>,
) -> HttpResponse {
    let routine_id = path.into_inner();
    match repo.get_training_days(&routine_id).await {
        Ok(training_days) => HttpResponse::Ok().json(training_days),
        Err(e) => HttpResponse::NotFound().body(format!("Internal server error: {:?}", e)),
    }
}

async fn create_training_day<R: RoutinesRepository>(
    create_training_day: web::Json<CreateTrainingDay>,
    repo: web::Data<R>,
) -> HttpResponse {
    match repo.create_training_day(&create_training_day).await {
        Ok(day) => HttpResponse::Ok().json(day),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Internal server error: {:?}", e))
        }
    }
}
// EXERCISES
async fn get_exercises<R: RoutinesRepository>(repo: web::Data<R>) -> HttpResponse {
    match repo.get_exercises().await {
        Ok(exercises) => HttpResponse::Ok().json(exercises),
        Err(e) => HttpResponse::NotFound().body(format!("Internal server error: {:?}", e)),
    }
}

async fn create_exercise<R: RoutinesRepository>(
    create_exercise: web::Json<CreateExercise>,
    repo: web::Data<R>,
) -> HttpResponse {
    match repo.create_exercise(&create_exercise).await {
        Ok(exercise) => HttpResponse::Ok().json(exercise),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Internal server error: {:?}", e))
        }
    }
}

async fn add_exercise_to_training_day<R: RoutinesRepository>(
    path: web::Path<(Uuid, Uuid)>,
    repo: web::Data<R>,
) -> HttpResponse {
    let (exercise_id, day_id) = path.into_inner();
    match repo
        .add_exercise_to_training_day(&exercise_id, &day_id)
        .await
    {
        Ok(exercise_day_link) => HttpResponse::Ok().json(exercise_day_link),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Internal server error: {:?}", e))
        }
    }
}

// async fn get<R: RoutineRepository>(routine_id: web::Path<Uuid>, repo: web::Data<R>) -> HttpResponse {
//     match repo.get_routine(&routine_id).await {
//         Ok(routine) => HttpResponse::Ok().json(routine),
//         Err(_) => HttpResponse::NotFound().body("Not found"),
//     }
