pub use postgres_routines_repository::PostgresRoutinesRepository;
use shared::models::{
    CreateExercise, CreateRoutine, CreateTrainingDay, Exercise, ExerciseToTrainingDay, Routine,
    TrainingDay,
};
use uuid::Uuid;

pub type Error = String;
pub type RoutineResult<T> = Result<T, Error>;
pub type DeleteRoutineResult<T> = Result<T, Error>;
pub type TrainingDayResult<T> = Result<T, Error>;
pub type ExerciseResult<T> = Result<T, Error>;
pub type ExerciseToTrainingDayResult<T> = Result<T, Error>;

#[async_trait::async_trait]
pub trait RoutinesRepository: Send + Sync + 'static {
    // routines
    async fn get_routines(&self) -> RoutineResult<Vec<Routine>>;
    async fn get_active_routine(&self) -> RoutineResult<Routine>;
    // async fn get_routine(&self, id: &Uuid) -> RoutineResult<Routine>;
    async fn create_routine(&self, create_routine: &CreateRoutine) -> RoutineResult<Routine>;
    async fn delete_routine(&self, routine_id: &Uuid) -> RoutineResult<Uuid>;

    // training days
    async fn get_training_days(&self, routine_id: &Uuid) -> TrainingDayResult<Vec<TrainingDay>>;
    async fn create_training_day(
        &self,
        create_training_day: &CreateTrainingDay,
    ) -> TrainingDayResult<TrainingDay>;

    // exercises
    async fn get_exercises(&self) -> ExerciseResult<Vec<Exercise>>;
    async fn create_exercise(&self, id: &CreateExercise) -> ExerciseResult<Exercise>;
    async fn add_exercise_to_training_day(
        &self,
        exercise_id: &Uuid,
        day_id: &Uuid,
    ) -> ExerciseToTrainingDayResult<ExerciseToTrainingDay>;

    // async fn update_routine(&self, id: &Routine) -> RoutineResult<Routine>;
    // async fn delete_routine(&self, id: &Uuid) -> RoutineResult<Uuid>;
}

mod postgres_routines_repository;
