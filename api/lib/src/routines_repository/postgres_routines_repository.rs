use super::{
    CustomError, ExerciseToTrainingDayResult, RoutineResult, RoutinesRepository,
    SelectedExercisesWithLinkIdResult, TrainingDayResult,
};
use shared::models::{
    CreateExercise, CreateRoutine, CreateTrainingDay, Exercise, ExerciseToTrainingDay,
    ExerciseWithLinkId, Routine, TrainingDay,
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
      SELECT routine_id, name, description, is_active, created_at, updated_at
      FROM routines
      "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn get_active_routines(&self) -> RoutineResult<Vec<Routine>> {
        sqlx::query_as::<_, Routine>(
            r#"
      SELECT routine_id, name, description, is_active, created_at, updated_at
      FROM routines
      WHERE is_active = true
      "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn create_routine(&self, create_routine: &CreateRoutine) -> RoutineResult<Routine> {
        sqlx::query_as::<_, Routine>(
            r#"
      INSERT INTO routines (name, description, is_active)
      VALUES ($1, $2, $3)
      RETURNING routine_id, name, description,is_active, created_at, updated_at
      "#,
        )
        .bind(&create_routine.name)
        .bind(&create_routine.description)
        .bind(&create_routine.is_active)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn delete_routine(&self, routine_id: &uuid::Uuid) -> RoutineResult<Uuid> {
        sqlx::query_scalar::<_, Uuid>(
            r#"
      DELETE FROM routines
      WHERE routine_id = $1
        RETURNING routine_id
      "#,
        )
        .bind(&routine_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn update_routine(&self, routine: &Routine) -> RoutineResult<Routine> {
        sqlx::query_as::<_, Routine>(
            r#"
      UPDATE routines
      SET name = $1, description = $2, is_active = $3
      WHERE routine_id = $4
      RETURNING routine_id, name, description, is_active, created_at, updated_at
      "#,
        )
        .bind(&routine.name)
        .bind(&routine.description)
        .bind(&routine.is_active)
        .bind(&routine.routine_id)
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

    async fn delete_training_day(&self, day_id: &Uuid) -> TrainingDayResult<Option<Uuid>> {
        let mut transaction = self.pool.begin().await.map_err(|e| e.to_string())?;

        // Step 1: Delete references in the link table (ExerciseTrainingDayLink)
        let link_table_result =
            sqlx::query("DELETE FROM ExerciseTrainingDayLink WHERE day_id = $1")
                .bind(day_id)
                .execute(transaction.as_mut())
                .await;

        if let Err(e) = link_table_result {
            transaction.rollback().await.map_err(|e| e.to_string())?;
            return Err(format!(
                "Error deleting day references from link table: {}",
                e
            ));
        }

        log::info!("Deleting rows with day_id: {}", day_id);

        // Step 2: Delete the TrainingDay from the TrainingDays table
        let training_days_result = sqlx::query("DELETE FROM TrainingDays WHERE day_id = $1 ")
            .bind(day_id)
            .execute(transaction.as_mut())
            .await;

        // Commit or rollback the transaction based on the result of the second query
        match training_days_result {
            Ok(result) => {
                transaction.commit().await.map_err(|e| e.to_string())?;
                // Check if any rows were deleted
                if result.rows_affected() > 0 {
                    Ok(Some(*day_id))
                } else {
                    Ok(None)
                }
            }
            Err(e) => {
                transaction.rollback().await.map_err(|rollback_err| {
                    format!(
                        "Error rolling back transaction: {}. Original error: {}",
                        rollback_err, e
                    )
                })?;
                Err(format!("Error deleting day: {}", e))
            }
        }
    }

    async fn create_training_days(
        &self,
        create_training_days: &[CreateTrainingDay],
    ) -> TrainingDayResult<Vec<TrainingDay>> {
        if create_training_days.is_empty() {
            return Ok(vec![]);
        }

        // Create a Vec to store the results
        let mut results = Vec::with_capacity(create_training_days.len());

        // Execute the query using the database pool
        for create_training_day in create_training_days {
            let query = format!(
                r#"
        INSERT INTO trainingdays (day_name, routine_id)
        VALUES ($1, $2)
        RETURNING day_id, day_name, routine_id, created_at, updated_at
        "#,
            );

            let result = sqlx::query_as::<_, TrainingDay>(&query)
                .bind(&create_training_day.day_name)
                .bind(&create_training_day.routine_id)
                .fetch_one(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

            results.push(result);
        }

        Ok(results)
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

    async fn search_exercises(&self, name: &String) -> TrainingDayResult<Vec<Exercise>> {
        sqlx::query_as::<_, Exercise>(
            r#"
      SELECT exercise_id, exercise_name, exercise_description, created_at, updated_at
      FROM exercises
      WHERE exercise_name ILIKE $1
      "#,
        )
        .bind(format!("%{}%", name))
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
      INSERT INTO Exercises (exercise_name, exercise_description)
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

    async fn create_exercises(
        &self,
        exercises: &[CreateExercise],
    ) -> TrainingDayResult<Vec<Exercise>> {
        if exercises.is_empty() {
            return Ok(vec![]);
        }

        // Create a Vec to store the results
        let mut results = Vec::with_capacity(exercises.len());

        // Execute the query using the database pool
        for exercise in exercises {
            let query = format!(
                r#"
        INSERT INTO Exercises (exercise_name, exercise_description)
        VALUES ($1, $2)
        RETURNING exercise_id, exercise_name, exercise_description, created_at, updated_at
        "#,
            );

            let result = sqlx::query_as::<_, Exercise>(&query)
                .bind(&exercise.exercise_name)
                .bind(&exercise.exercise_description)
                .fetch_one(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

            results.push(result);
        }

        Ok(results)
    }

    async fn add_exercise_to_training_day(
        &self,
        exercise_id: &uuid::Uuid,
        day_id: &uuid::Uuid,
    ) -> ExerciseToTrainingDayResult<ExerciseToTrainingDay> {
        sqlx::query_as::<_, ExerciseToTrainingDay>(
            r#"
      INSERT INTO ExerciseTrainingDayLink (exercise_id, day_id)
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

    async fn get_exercises_for_training_day(
        &self,
        day_id: &uuid::Uuid,
    ) -> SelectedExercisesWithLinkIdResult<Vec<ExerciseWithLinkId>> {
        sqlx::query_as(
            r#"SELECT 
            e.exercise_id,
            e.exercise_name,
            e.exercise_description,
            e.created_at,
            e.updated_at,
            l.link_id AS "link_id"
        FROM exercises e
        JOIN ExerciseTrainingDayLink l
        ON e.exercise_id = l.exercise_id
        WHERE l.day_id = $1"#,
        )
        .bind(&day_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn remove_exercise_from_training_day(
        &self,
        link_id: &uuid::Uuid,
    ) -> ExerciseToTrainingDayResult<Uuid> {
        sqlx::query_scalar::<_, Uuid>(
            r#"
      DELETE FROM ExerciseTrainingDayLink
      WHERE link_id = $1
      RETURNING link_id
      "#,
        )
        .bind(&link_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn get_link_table_data(&self) -> ExerciseToTrainingDayResult<Vec<ExerciseToTrainingDay>> {
        sqlx::query_as::<_, ExerciseToTrainingDay>(
            r#"
      SELECT link_id, exercise_id, day_id, created_at, updated_at
      FROM ExerciseTrainingDayLink
      "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }
}
