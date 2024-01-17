use std::ptr::null;

use super::{ExerciseToTrainingDayResult, RoutineResult, RoutinesRepository, TrainingDayResult};
use shared::models::{
    CreateExercise, CreateRoutine, CreateTrainingDay, Exercise, ExerciseToTrainingDay, Routine,
    TrainingDay,
};
use uuid::Uuid;

pub struct PostgresRoutinesRepository {
    pool: sqlx::PgPool,
}

impl PostgresRoutinesRepository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl RoutinesRepository for PostgresRoutinesRepository {
    async fn get_routines(&self) -> RoutineResult<Vec<Routine>> {
        sqlx::query_as::<_, Routine>(
            r#"
      SELECT routine_id, name, description, created_at, updated_at
      FROM routines
      "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn get_active_routine(&self) -> RoutineResult<Routine> {
        sqlx::query_as::<_, Routine>(
            r#"
      SELECT routine_id, name, description, is_active, created_at, updated_at
      FROM routines
      WHERE is_active = true
      "#,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn create_routine(&self, create_routine: &CreateRoutine) -> RoutineResult<Routine> {
        sqlx::query_as::<_, Routine>(
            r#"
      INSERT INTO routines (name, description)
      VALUES ($1, $2)
      RETURNING routine_id, name, description, created_at, updated_at
      "#,
        )
        .bind(&create_routine.name)
        .bind(&create_routine.description)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn delete_routine(&self, routine_id: &uuid::Uuid) -> RoutineResult<Uuid> {
        sqlx::query_scalar::<_, Uuid>(
            r#"
      DELETE FROM routines
      WHERE routine_id = $1
      "#,
        )
        .bind(&routine_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    // async fn get_routine(&self, routine_id: &uuid::Uuid) -> RoutineResult<Routine> {
    //     sqlx::query_as::<_, Routine>(
    //         r#"
    //   SELECT id, title, description, created_at, updated_at
    //   FROM routines
    //   WHERE id = $1
    //   "#,
    //     )
    //     .bind(routine_id)
    //     .fetch_one(&self.pool)
    //     .await
    //     .map_err(|e| e.to_string())
    // }

    async fn get_training_days(
        &self,
        routine_id: &uuid::Uuid,
    ) -> TrainingDayResult<Vec<TrainingDay>> {
        sqlx::query_as::<_, TrainingDay>(
            r#"
      SELECT day_id, day_name, routine_id, created_at, updated_at
      FROM trainingdays
      WHERE routine_id = $1
      "#,
        )
        .bind(routine_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn create_training_day(
        &self,
        create_training_day: &CreateTrainingDay,
    ) -> TrainingDayResult<TrainingDay> {
        sqlx::query_as::<_, TrainingDay>(
            r#"
      INSERT INTO trainingdays (day_name, routine_id)
      VALUES ($1, $2)
      RETURNING day_id, day_name, routine_id, created_at, updated_at
      "#,
        )
        .bind(&create_training_day.day_name)
        .bind(&create_training_day.routine_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn get_exercises(&self) -> TrainingDayResult<Vec<Exercise>> {
        sqlx::query_as::<_, Exercise>(
            r#"
      SELECT exercise_id, exercise_name, exercise_description, created_at, updated_at
      FROM exercises
      "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn create_exercise(
        &self,
        create_exercise: &CreateExercise,
    ) -> TrainingDayResult<Exercise> {
        sqlx::query_as::<_, Exercise>(
            r#"
      INSERT INTO exercises (exercise_name, exercise_description)
      VALUES ($1, $2)
      RETURNING exercise_id, exercise_name, exercise_description, created_at, updated_at
      "#,
        )
        .bind(&create_exercise.exercise_name)
        .bind(&create_exercise.exercise_description)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn add_exercise_to_training_day(
        &self,
        exercise_id: &uuid::Uuid,
        day_id: &uuid::Uuid,
    ) -> ExerciseToTrainingDayResult<ExerciseToTrainingDay> {
        sqlx::query_as::<_, ExerciseToTrainingDay>(
            r#"
      INSERT INTO ExerciseTrainiDayLink (exercise_id, day_id)
      VALUES ($1, $2)
      RETURNING link_id, exercise_id, day_id, created_at, updated_at
      "#,
        )
        .bind(&exercise_id)
        .bind(&day_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }
}
