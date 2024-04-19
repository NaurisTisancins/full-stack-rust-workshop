use actix_web::{
    dev::ServiceRequest,
    error::Error,
    web::{self, delete, get, post, put, scope, Data, Json, Path, Query, ServiceConfig},
    HttpMessage, HttpResponse,
};

use actix_web_httpauth::{
    extractors::{
        self,
        basic::BasicAuth,
        bearer::{self, BearerAuth},
        AuthenticationError,
    },
    middleware::HttpAuthentication,
};
use argonautica::{Hasher, Verifier};
use chrono::NaiveDateTime;
use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use jwt::VerifyWithKey;
use sha2::Sha256;
use shared::models::TokenClaims;

use shared::models::{
    AuthUser, CreateExercise, CreateRoutine, CreateTrainingDay, CreateUser, Routine, SearchQuery,
    SetPerformancePayload, UserNoPassword,
};
use uuid::Uuid;

use crate::routines_repository::RoutinesRepository;

async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let jwt_secret: String = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set!");
    let key: Hmac<Sha256> = Hmac::new_from_slice(jwt_secret.as_bytes()).unwrap();
    let token_string = credentials.token();

    let claims: Result<TokenClaims, &str> = token_string
        .verify_with_key(&key)
        .map_err(|_| "Invalid token");

    match claims {
        Ok(value) => {
            req.extensions_mut().insert(value);
            Ok(req)
        }
        Err(_) => {
            let config = req
                .app_data::<bearer::Config>()
                .cloned()
                .unwrap_or_default()
                .scope("");

            Err((AuthenticationError::from(config).into(), req))
        }
    }
}

pub fn service<R: RoutinesRepository>(cfg: &mut ServiceConfig) {
    let bearer_middleware = HttpAuthentication::bearer(validator);
    cfg.service(
        scope("/v1")
            .service(
                scope("/users")
                    .route("/auth", get().to(basic_auth::<R>))
                    .route("/all", get().to(get_users::<R>))
                    .route("/create", post().to(create_user::<R>)),
            )
            .service(
                scope("")
                    .wrap(bearer_middleware)
                    .service(
                        scope("/routines")
                            .route("", get().to(get_all_routines::<R>))
                            .route("/active", get().to(get_active_routine::<R>))
                            .route("", post().to(create_routine::<R>))
                            .route("", put().to(update_routine::<R>))
                            .route("/{routine_id}", delete().to(delete_routine::<R>)),
                    )
                    .service(
                        scope("/training_days")
                            .route(
                                "", // post new training days as an array
                                post().to(create_training_days::<R>),
                            )
                            .route(
                                "/with_exercises/{routine_id}",
                                get().to(get_training_days_with_exercises::<R>),
                            )
                            .route(
                                "/{routine_id}", // post new training day
                                post().to(create_training_day::<R>),
                            )
                            .route(
                                "/{day_id}", // DELETE training day
                                delete().to(delete_training_day::<R>),
                            )
                            .route("/{routine_id}", get().to(get_training_days::<R>)),
                    )
                    .service(
                        scope("/exercises")
                            .route("", post().to(create_exercise::<R>))
                            .route("/bulk", post().to(create_exercises::<R>))
                            .route("", get().to(get_exercises::<R>))
                            .route("/search", get().to(search_exercises::<R>))
                            .route(
                                "/{exercise_id}/{day_id}",
                                post().to(add_exercise_to_training_day::<R>),
                            )
                            .route("/{day_id}", get().to(get_exercises_for_training_day::<R>))
                            .route(
                                "/{link_id}",
                                delete().to(delete_exercise_from_training_day::<R>),
                            ),
                    )
                    .service(
                        scope("/session")
                            .route("/{day_id}", post().to(create_session::<R>))
                            .route("/{day_id}/all", get().to(get_sessions_by_day_id::<R>))
                            .route(
                                "/{day_id}",
                                get().to(get_sessions_with_exercises_by_day_id::<R>),
                            )
                            .route(
                                "/in_progress/{routine_id}",
                                get().to(get_session_in_progress::<R>),
                            )
                            .route(
                                "/all/{routine_id}",
                                get().to(get_all_sessions_by_routine_id::<R>),
                            )
                            .route(
                                "/{session_id}/{exercise_id}",
                                post().to(add_set_performance_to_session::<R>),
                            )
                            .route(
                                "/{performance_id}",
                                delete().to(remove_set_performance_from_session::<R>),
                            )
                            .route("/end/{session_id}", put().to(end_session::<R>)),
                    )
                    .route("/debug/link_table", get().to(get_link_table_data::<R>))
                    .route("/debug/clear_data", get().to(clear_data::<R>)),
            ),
    );
}

//Auth
async fn basic_auth<R: RoutinesRepository>(credentials: BasicAuth, repo: Data<R>) -> HttpResponse {
    println!("hello from basic auth");
    let jwt_secret: Hmac<Sha256> = Hmac::new_from_slice(
        std::env::var("JWT_SECRET")
            .expect("JWT_SECRET must be set!")
            .as_bytes(),
    )
    .unwrap();
    let username = credentials.user_id();
    let password = credentials.password();
    println!("username: {}", username);
    match password {
        None => HttpResponse::Unauthorized().json("Must provide username and password"),
        Some(pass) => match repo.get_user(&username).await {
            Ok(user) => {
                let hash_secret = std::env::var("HASH_SECRET").expect("HASH_SECRET must be set!");
                let mut verifier = Verifier::default();
                let is_valid = verifier
                    .with_hash(user.password)
                    .with_password(pass)
                    .with_secret_key(hash_secret)
                    .verify()
                    .unwrap();

                if is_valid {
                    let claims = TokenClaims {
                        token_id: user.user_id,
                    };
                    let token_str = claims.sign_with_key(&jwt_secret).unwrap();
                    HttpResponse::Ok().json(token_str)
                } else {
                    HttpResponse::Unauthorized().json("Incorrect username or password")
                }
            }
            Err(error) => HttpResponse::InternalServerError().json(format!("{:?}", error)),
        },
    }
}

// USERS
async fn create_user<R: RoutinesRepository>(
    create_user: Json<CreateUser>,
    repo: Data<R>,
) -> HttpResponse {
    let user: CreateUser = create_user.into_inner();

    let hash_secret = std::env::var("HASH_SECRET").expect("HASH_SECRET must be set!");
    let mut hasher = Hasher::default();
    let hash = hasher
        .with_password(user.password)
        .with_secret_key(hash_secret)
        .hash()
        .unwrap();

    let create_user = CreateUser {
        username: user.username.clone(),
        password: hash,
    };

    match repo.create_user(&create_user).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(error) => HttpResponse::InternalServerError().json(format!("{:?}", error)),
    }
}

async fn get_users<R: RoutinesRepository>(repo: Data<R>) -> HttpResponse {
    match repo.get_users().await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => HttpResponse::NotFound().body(format!("Internal server error: {:?}", e)),
    }
}

// ROUTINES
async fn get_all_routines<R: RoutinesRepository>(repo: Data<R>) -> HttpResponse {
    match repo.get_routines().await {
        Ok(routines) => HttpResponse::Ok().json(routines),
        Err(e) => HttpResponse::NotFound().body(format!("Internal server error: {:?}", e)),
    }
}

async fn get_active_routine<R: RoutinesRepository>(repo: Data<R>) -> HttpResponse {
    match repo.get_active_routines().await {
        Ok(routine_id) => HttpResponse::Ok().json(routine_id),
        Err(e) => HttpResponse::NotFound().body(format!("Internal server error: {:?}", e)),
    }
}

async fn create_routine<R: RoutinesRepository>(
    create_routine: Json<CreateRoutine>,
    repo: Data<R>,
) -> HttpResponse {
    match repo.create_routine(&create_routine).await {
        Ok(routine) => HttpResponse::Ok().json(routine),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Internal server error: {:?}", e))
        }
    }
}

async fn update_routine<R: RoutinesRepository>(
    routine: Json<Routine>,
    repo: Data<R>,
) -> HttpResponse {
    match repo.update_routine(&routine).await {
        Ok(routine) => HttpResponse::Ok().json(routine),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Internal server error: {:?}", e))
        }
    }
}

async fn delete_routine<R: RoutinesRepository>(
    routine_id: Path<Uuid>,
    repo: Data<R>,
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
async fn get_training_days<R: RoutinesRepository>(path: Path<Uuid>, repo: Data<R>) -> HttpResponse {
    let routine_id = path.into_inner();
    match repo.get_training_days(&routine_id).await {
        Ok(training_days) => HttpResponse::Ok().json(training_days),
        Err(e) => HttpResponse::NotFound().body(format!("Internal server error: {:?}", e)),
    }
}

async fn get_training_days_with_exercises<R: RoutinesRepository>(
    path: Path<Uuid>,
    repo: Data<R>,
) -> HttpResponse {
    let routine_id = path.into_inner();
    match repo.get_training_days_with_exercises(&routine_id).await {
        Ok(training_days) => HttpResponse::Ok().json(training_days),
        Err(e) => HttpResponse::NotFound().body(format!("Internal server error: {:?}", e)),
    }
}

async fn create_training_day<R: RoutinesRepository>(
    create_training_day: Json<CreateTrainingDay>,
    repo: Data<R>,
) -> HttpResponse {
    match repo.create_training_day(&create_training_day).await {
        Ok(day) => HttpResponse::Ok().json(day),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Internal server error: {:?}", e))
        }
    }
}

async fn delete_training_day<R: RoutinesRepository>(
    path: Path<Uuid>,
    repo: Data<R>,
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
    create_training_days: Json<Vec<CreateTrainingDay>>,
    repo: Data<R>,
) -> HttpResponse {
    match repo.create_training_days(&create_training_days).await {
        Ok(days) => HttpResponse::Ok().json(days),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Internal server error: {:?}", e))
        }
    }
}

// EXERCISES
async fn get_exercises<R: RoutinesRepository>(repo: Data<R>) -> HttpResponse {
    match repo.get_exercises().await {
        Ok(exercises) => HttpResponse::Ok().json(exercises),
        Err(e) => HttpResponse::NotFound().body(format!("Internal server error: {:?}", e)),
    }
}

async fn search_exercises<R: RoutinesRepository>(
    query: Query<SearchQuery>,
    repo: Data<R>,
) -> HttpResponse {
    let name: &String = &query.name;
    match repo.search_exercises(&name).await {
        Ok(exercises) => HttpResponse::Ok().json(exercises),
        Err(e) => HttpResponse::NotFound().body(format!("Internal server error: {:?}", e)),
    }
}

async fn create_exercise<R: RoutinesRepository>(
    create_exercise: Json<CreateExercise>,
    repo: Data<R>,
) -> HttpResponse {
    match repo.create_exercise(&create_exercise).await {
        Ok(exercise) => HttpResponse::Ok().json(exercise),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Internal server error: {:?}", e))
        }
    }
}

async fn create_exercises<R: RoutinesRepository>(
    create_exercises: Json<Vec<CreateExercise>>,
    repo: Data<R>,
) -> HttpResponse {
    match repo.create_exercises(&create_exercises).await {
        Ok(exercises) => HttpResponse::Ok().json(exercises),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Internal server error: {:?}", e))
        }
    }
}

async fn add_exercise_to_training_day<R: RoutinesRepository>(
    path: Path<(Uuid, Uuid)>,
    repo: Data<R>,
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
    path: Path<Uuid>,
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

//SESSIONS
async fn create_session<R: RoutinesRepository>(
    path: web::Path<Uuid>,
    repo: web::Data<R>,
) -> HttpResponse {
    let day_id = path.into_inner();
    match repo.create_session(&day_id).await {
        Ok(session_with_exercises) => HttpResponse::Ok().json(session_with_exercises),
        Err(e) => HttpResponse::NotFound().body(format!("Internal server error: {:?}", e)),
    }
}

async fn get_sessions_by_day_id<R: RoutinesRepository>(
    path: web::Path<Uuid>,
    repo: web::Data<R>,
) -> HttpResponse {
    let day_id = path.into_inner();
    match repo.get_all_sessions_by_day_id(&day_id).await {
        Ok(sessions) => HttpResponse::Ok().json(sessions),
        Err(e) => HttpResponse::NotFound().body(format!("Internal server error: {:?}", e)),
    }
}

async fn get_sessions_with_exercises_by_day_id<R: RoutinesRepository>(
    path: web::Path<Uuid>,
    repo: web::Data<R>,
) -> HttpResponse {
    let day_id = path.into_inner();
    match repo.get_sessions_with_exercises(&day_id).await {
        Ok(sessions_with_exercises) => HttpResponse::Ok().json(sessions_with_exercises),
        Err(e) => HttpResponse::NotFound().body(format!("Internal server error {:?}", e)),
    }
}

async fn get_session_in_progress<R: RoutinesRepository>(
    path: web::Path<Uuid>,
    repo: web::Data<R>,
) -> HttpResponse {
    let routine_id = path.into_inner();
    match repo.get_session_in_progress(&routine_id).await {
        Ok(session_with_exercises) => HttpResponse::Ok().json(session_with_exercises),
        Err(e) => HttpResponse::NotFound().body(format!("Internal server error: {:?}", e)),
    }
}

async fn get_all_sessions_by_routine_id<R: RoutinesRepository>(
    path: web::Path<Uuid>,
    repo: web::Data<R>,
) -> HttpResponse {
    let routine_id = path.into_inner();
    match repo.get_all_sessions_by_routine_id(&routine_id).await {
        Ok(sessions) => HttpResponse::Ok().json(sessions),
        Err(e) => HttpResponse::NotFound().body(format!("Internal server error: {:?}", e)),
    }
}

async fn end_session<R: RoutinesRepository>(
    path: web::Path<Uuid>,
    repo: web::Data<R>,
) -> HttpResponse {
    let session_id = path.into_inner();
    match repo.end_session(&session_id).await {
        Ok(session_id) => HttpResponse::Ok().json(session_id),
        Err(e) => HttpResponse::NotFound().body(format!("Internal server error: {:?}", e)),
    }
}

async fn add_set_performance_to_session<R: RoutinesRepository>(
    path: web::Path<(Uuid, Uuid)>,
    set_performance: web::Json<SetPerformancePayload>,
    repo: web::Data<R>,
) -> HttpResponse {
    let (session_id, exercise_id) = path.into_inner();
    match repo
        .add_set_performance_to_session(&session_id, &exercise_id, &set_performance)
        .await
    {
        Ok(session_id) => HttpResponse::Ok().json(session_id),
        Err(e) => HttpResponse::NotFound().body(format!("Internal server error: {:?}", e)),
    }
}

async fn remove_set_performance_from_session<R: RoutinesRepository>(
    path: web::Path<Uuid>,
    repo: web::Data<R>,
) -> HttpResponse {
    let performance_id = path.into_inner();
    match repo
        .remove_set_performance_from_session(&performance_id)
        .await
    {
        Ok(session_id) => HttpResponse::Ok().json(session_id),
        Err(e) => HttpResponse::NotFound().body(format!("Internal server error: {:?}", e)),
    }
}

async fn clear_data<R: RoutinesRepository>(repo: web::Data<R>) -> HttpResponse {
    match repo.clear_data().await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::NotFound().body(format!("Internal server error: {:?}", e)),
    }
}
