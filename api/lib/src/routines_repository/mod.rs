pub use postgres_routines_repository::PostgresRoutinesRepository;
use shared::models::{
    CreateExercise, CreateRoutine, CreateTrainingDay, CreateUser, Exercise, ExerciseToTrainingDay,
    ExerciseWithLinkId, Routine, Session, SessionWithExercisePerformance, SessionWithExercises,
    SetPerformance, SetPerformancePayload, TrainingDay, TrainingDayWithExercises, User,
};

use uuid::Uuid;

pub type CustomError = String;

pub type RoutineResult<T> = Result<T, CustomError>;
pub type DeleteRoutineResult<T> = Result<T, CustomError>;
pub type TrainingDayResult<T> = Result<T, CustomError>;
pub type ExerciseResult<T> = Result<T, CustomError>;
pub type SelectedExercisesWithLinkIdResult<T> = Result<T, CustomError>;
pub type ExerciseToTrainingDayResult<T> = Result<T, CustomError>;

#[derive(Debug)]
pub enum SessionError {
    PreviousSessionInProgress,
    NoExercisesFound,
    Error(String),
}

impl From<String> for SessionError {
    fn from(msg: String) -> Self {
        SessionError::Error(msg)
    }
}

// Implement conversion for `&str` as well
impl<'a> From<&'a str> for SessionError {
    fn from(msg: &'a str) -> Self {
        SessionError::Error(msg.to_string())
    }
}

pub type SessionResult<T> = Result<T, SessionError>;

#[async_trait::async_trait]
pub trait RoutinesRepository: Send + Sync + 'static {
    //users
    async fn create_user(&self, create_user: &CreateUser) -> RoutineResult<User>;
    async fn get_users(&self) -> RoutineResult<Vec<User>>;
    async fn get_user(&self, username: &str) -> RoutineResult<User>;
    // routines
    async fn get_routines(&self) -> RoutineResult<Vec<Routine>>;
    async fn get_active_routines(&self) -> RoutineResult<Vec<Routine>>;
    // async fn get_routine(&self, id: &Uuid) -> RoutineResult<Routine>;
    async fn create_routine(&self, create_routine: &CreateRoutine) -> RoutineResult<Routine>;
    async fn update_routine(&self, routine: &Routine) -> RoutineResult<Routine>;
    async fn delete_routine(&self, routine_id: &Uuid) -> RoutineResult<Uuid>;

    // training days
    async fn get_training_days(&self, routine_id: &Uuid) -> TrainingDayResult<Vec<TrainingDay>>;
    async fn get_training_days_with_exercises(
        &self,
        day_id: &Uuid,
    ) -> Result<Vec<TrainingDayWithExercises>, sqlx::Error>;
    async fn create_training_day(
        &self,
        create_training_day: &CreateTrainingDay,
    ) -> TrainingDayResult<TrainingDay>;
    async fn delete_training_day(&self, day_id: &Uuid) -> TrainingDayResult<Option<Uuid>>;
    async fn create_training_days(
        &self,
        create_training_days: &[CreateTrainingDay],
    ) -> TrainingDayResult<Vec<TrainingDay>>;

    // exercises
    async fn get_exercises(&self) -> ExerciseResult<Vec<Exercise>>;
    async fn search_exercises(&self, name: &String) -> ExerciseResult<Vec<Exercise>>;
    async fn create_exercise(&self, id: &CreateExercise) -> ExerciseResult<Exercise>;
    async fn create_exercises(&self, exercises: &[CreateExercise])
        -> ExerciseResult<Vec<Exercise>>;
    async fn add_exercise_to_training_day(
        &self,
        exercise_id: &Uuid,
        day_id: &Uuid,
    ) -> ExerciseToTrainingDayResult<ExerciseToTrainingDay>;
    async fn get_exercises_for_training_day(
        &self,
        day_id: &Uuid,
    ) -> SelectedExercisesWithLinkIdResult<Vec<ExerciseWithLinkId>>;

    async fn remove_exercise_from_training_day(
        &self,
        link_id: &Uuid,
    ) -> ExerciseToTrainingDayResult<Uuid>;

    async fn get_link_table_data(&self) -> ExerciseToTrainingDayResult<Vec<ExerciseToTrainingDay>>;

    async fn is_previous_session_in_progress(&self, day_id: &Uuid) -> SessionResult<bool>;
    async fn create_session(&self, day_id: &Uuid) -> SessionResult<SessionWithExercisePerformance>;
    async fn get_all_sessions_by_day_id(&self, day_id: &Uuid) -> SessionResult<Vec<Session>>;
    async fn get_sessions_with_exercises(
        &self,
        day_id: &Uuid,
    ) -> SessionResult<Vec<SessionWithExercises>>;
    async fn get_session_in_progress(
        &self,
        routine_id: &Uuid,
    ) -> SessionResult<Option<SessionWithExercisePerformance>>;
    async fn get_all_sessions_by_routine_id(
        &self,
        routine_id: &Uuid,
    ) -> SessionResult<Vec<Session>>;
    async fn end_session(&self, session_id: &Uuid) -> SessionResult<Uuid>;

    async fn add_set_performance_to_session(
        &self,
        session_id: &Uuid,
        exercise_id: &Uuid,
        set_performance: &SetPerformancePayload,
    ) -> SessionResult<SetPerformance>;

    async fn remove_set_performance_from_session(
        &self,
        performance_id: &Uuid,
    ) -> SessionResult<Uuid>;

    async fn clear_data(&self) -> Result<(), sqlx::Error>;
}

mod postgres_routines_repository;
