use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse,
};

use shared::models::{
    CreateExercise, CreateRoutine, CreateTrainingDay, Exercise, Routine, SearchQuery, TrainingDay,
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
            .route("/routines", web::put().to(update_routine::<R>))
            .route(
                "/routines/{routine_id}",
                web::delete().to(delete_routine::<R>),
            )
            .route(
                "/training_days", // post new training days as an array
                web::post().to(create_training_days::<R>),
            )
            .route(
                "/training_days/{routine_id}", // post new training day
                web::post().to(create_training_day::<R>),
            )
            .route(
                "/training_days/{day_id}", // DELETE training day
                web::delete().to(delete_training_day::<R>),
            )
            .route(
                "/training_days/{routine_id}",
                web::get().to(get_training_days::<R>),
            )
            .route("/exercises", web::post().to(create_exercise::<R>))
            .route("/exercises/bulk", web::post().to(create_exercises::<R>))
            .route("/exercises", web::get().to(get_exercises::<R>))
            .route("/exercises/search", web::get().to(search_exercises::<R>))
            .route(
                "/exercises/{exercise_id}/{day_id}",
                web::post().to(add_exercise_to_training_day::<R>),
            )
            .route(
                "/exercises/{day_id}",
                web::get().to(get_exercises_for_training_day::<R>),
            )
            .route(
                "exercises/{link_id}",
                web::delete().to(delete_exercise_from_training_day::<R>),
            )
            .route("/debug/link_table", web::get().to(get_link_table_data::<R>)),
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
    match repo.get_active_routines().await {
        Ok(routine_id) => HttpResponse::Ok().json(routine_id),
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

async fn update_routine<R: RoutinesRepository>(
    routine: web::Json<Routine>,
    repo: web::Data<R>,
) -> HttpResponse {
    match repo.update_routine(&routine).await {
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

async fn delete_training_day<R: RoutinesRepository>(
    path: web::Path<Uuid>,
    repo: web::Data<R>,
) -> HttpResponse {
    let day_id = path.into_inner();
    match repo.delete_training_day(&day_id).await {
        Ok(day_id) => HttpResponse::Ok().json(day_id),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Internal server error: {:?}", e))
        }
    }
}

async fn create_training_days<R: RoutinesRepository>(
    create_training_days: web::Json<Vec<CreateTrainingDay>>,
    repo: web::Data<R>,
) -> HttpResponse {
    match repo.create_training_days(&create_training_days).await {
        Ok(days) => HttpResponse::Ok().json(days),
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

async fn search_exercises<R: RoutinesRepository>(
    query: web::Query<SearchQuery>,
    repo: web::Data<R>,
) -> HttpResponse {
    let name: &String = &query.name;
    match repo.search_exercises(&name).await {
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

async fn create_exercises<R: RoutinesRepository>(
    create_exercises: web::Json<Vec<CreateExercise>>,
    repo: web::Data<R>,
) -> HttpResponse {
    match repo.create_exercises(&create_exercises).await {
        Ok(exercises) => HttpResponse::Ok().json(exercises),
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

async fn get_exercises_for_training_day<R: RoutinesRepository>(
    path: web::Path<Uuid>,
    repo: web::Data<R>,
) -> HttpResponse {
    let day_id = path.into_inner();
    match repo.get_exercises_for_training_day(&day_id).await {
        Ok(exercises) => HttpResponse::Ok().json(exercises),
        Err(e) => HttpResponse::NotFound().body(format!("Internal server error: {:?}", e)),
    }
}

async fn delete_exercise_from_training_day<R: RoutinesRepository>(
    path: web::Path<Uuid>,
    repo: web::Data<R>,
) -> HttpResponse {
    let link_id = path.into_inner();
    match repo.remove_exercise_from_training_day(&link_id).await {
        Ok(link_id) => HttpResponse::Ok().json(link_id),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Internal server error: {:?}", e))
        }
    }
}

async fn get_link_table_data<R: RoutinesRepository>(repo: web::Data<R>) -> HttpResponse {
    match repo.get_link_table_data().await {
        Ok(link_table_data) => HttpResponse::Ok().json(link_table_data),
        Err(e) => HttpResponse::NotFound().body(format!("Internal server error: {:?}", e)),
    }
}
