pub use postgres_routines_repository::PostgresRoutinesRepository;
use shared::models::{
    CreateExercise, CreateRoutine, CreateTrainingDay, Exercise, ExerciseToTrainingDay,
    ExerciseWithLinkId, Routine, TrainingDay, TrainingDayWithExercises,
};
use uuid::Uuid;

pub type CustomError = String;

pub type RoutineResult<T> = Result<T, CustomError>;
pub type DeleteRoutineResult<T> = Result<T, CustomError>;
pub type TrainingDayResult<T> = Result<T, CustomError>;
pub type ExerciseResult<T> = Result<T, CustomError>;
pub type SelectedExercisesWithLinkIdResult<T> = Result<T, CustomError>;
pub type ExerciseToTrainingDayResult<T> = Result<T, CustomError>;

#[async_trait::async_trait]
pub trait RoutinesRepository: Send + Sync + 'static {
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
    // async fn update_routine(&self, id: &Routine) -> RoutineResult<Routine>;
    // async fn delete_routine(&self, id: &Uuid) -> RoutineResult<Uuid>;
}

mod postgres_routines_repository;
