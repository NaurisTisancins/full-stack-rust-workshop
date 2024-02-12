use std::collections::HashMap;

use super::{
    ExerciseToTrainingDayResult, RoutineResult, RoutinesRepository,
    SelectedExercisesWithLinkIdResult, SessionError, SessionResult, TrainingDayResult,
};

use shared::models::{
    CreateExercise, CreateRoutine, CreateTrainingDay, Exercise, ExerciseToTrainingDay,
    ExerciseWithLinkId, Routine, Session, SessionWithExercises, SessionsWithExercisesQuery,
    TrainingDay, TrainingDayWithExercises, TrainingDayWithExercisesQuery,
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

    async fn get_training_days_with_exercises(
        &self,
        routine_id: &Uuid,
    ) -> Result<Vec<TrainingDayWithExercises>, sqlx::Error> {
        let query = sqlx::query_as::<_, TrainingDayWithExercisesQuery>(
            r#"
            SELECT
                td.day_id AS day_id,
                td.routine_id AS routine_id,
                td.day_name AS day_name,
                td.created_at AS created_at,
                td.updated_at AS updated_at,
                e.exercise_id AS exercise_id,
                e.exercise_name AS exercise_name,
                e.exercise_description AS exercise_description,
                etdl.link_id AS link_id
            FROM
                TrainingDays td
            LEFT JOIN
                ExerciseTrainingDayLink etdl ON td.day_id = etdl.day_id
            LEFT JOIN
                Exercises e ON etdl.exercise_id = e.exercise_id
            WHERE
                td.routine_id = $1
            "#,
        )
        .bind(routine_id);

        let rows = query.fetch_all(&self.pool).await?;

        // Group exercises by training day
        let mut training_days: HashMap<Uuid, TrainingDayWithExercises> = HashMap::new();

        for row in rows {
            let day_id = row.day_id;
            let routine_id = row.routine_id;
            let day_name = row.day_name;
            let created_at = row.created_at;
            let updated_at = row.updated_at;

            let exercise = match (
                row.exercise_id,
                row.exercise_name,
                row.exercise_description,
                row.link_id,
            ) {
                (
                    Some(exercise_id),
                    Some(exercise_name),
                    Some(exercise_description),
                    Some(link_id),
                ) => Some(ExerciseWithLinkId {
                    exercise_id,
                    exercise_name,
                    exercise_description,
                    link_id,
                    created_at,
                    updated_at,
                }),
                _ => None,
            };

            let training_day =
                training_days
                    .entry(day_id)
                    .or_insert_with(|| TrainingDayWithExercises {
                        day_id,
                        routine_id,
                        day_name,
                        exercises: Some(Vec::new()),
                        created_at,
                        updated_at,
                    });

            if let Some(exercise) = exercise {
                if let Some(exercises) = &mut training_day.exercises {
                    exercises.push(exercise);
                } else {
                    // If exercises is None, create a new Vec and push the exercise
                    training_day.exercises = Some(vec![exercise]);
                }
            }
        }

        // Convert HashMap values to Vec
        let result: Vec<TrainingDayWithExercises> = training_days.into_values().collect();

        Ok(result)
    }

    async fn is_previous_session_in_progress(&self, day_id: &Uuid) -> SessionResult<bool> {
        let query = sqlx::query_as::<_, (bool,)>(
            r#"
        SELECT EXISTS (
            SELECT 1
            FROM Sessions
            WHERE day_id = $1 AND in_progress = true
        ) AS previous_session_in_progress
        "#,
        )
        .bind(day_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string());

        // Extract the boolean value from the query result tuple
        let previous_session_in_progress = match query {
            Ok((value,)) => value,
            Err(_) => false,
        };

        Ok(previous_session_in_progress)
    }

    async fn create_session(&self, day_id: &Uuid) -> SessionResult<SessionWithExercises> {
        if self.is_previous_session_in_progress(day_id).await? {
            return Err(SessionError::PreviousSessionInProgress);
        }

        // Insert a new session into the database
        let session_query = sqlx::query_as::<_, Session>(
            r#"
        INSERT INTO Sessions (day_id)
        VALUES ($1)
        RETURNING session_id, day_id, in_progress, created_at, updated_at
        "#,
        )
        .bind(&day_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| SessionError::Error(e.to_string()))?;

        // Fetch exercises associated with the newly created session
        let exercises_query = sqlx::query_as::<_, ExerciseWithLinkId>(
            r#"
        SELECT
            e.exercise_id,
            etdl.link_id,
            e.exercise_name,
            e.exercise_description,
            e.created_at,
            e.updated_at
        FROM
            ExerciseTrainingDayLink etdl
        LEFT JOIN
            Exercises e ON etdl.exercise_id = e.exercise_id
        WHERE
            etdl.day_id = $1
        "#,
        )
        .bind(&day_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| SessionError::Error(e.to_string()))?;

        // Construct the SessionWithExercises struct
        let session_with_exercises = SessionWithExercises {
            session_id: session_query.session_id,
            day_id: session_query.day_id,
            in_progress: session_query.in_progress,
            exercises: exercises_query,
            created_at: session_query.created_at,
            updated_at: session_query.updated_at,
        };

        Ok(session_with_exercises)
    }

    async fn get_all_sessions_by_day_id(&self, day_id: &Uuid) -> SessionResult<Vec<Session>> {
        sqlx::query_as::<_, Session>(
            r#"
            SELECT * 
            FROM Sessions
        WHERE Sessions.day_id = $1
            "#,
        )
        .bind(&day_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| SessionError::Error(e.to_string()))
    }

    async fn get_sessions_with_exercises(
        &self,
        day_id: &Uuid,
    ) -> SessionResult<Vec<SessionWithExercises>> {
        let query = sqlx::query_as::<_, SessionsWithExercisesQuery>(
            r#"
        SELECT 
            s.session_id,
            s.day_id,
            s.in_progress,
            e.exercise_id,
            e.exercise_name,
            e.exercise_description,
            etdl.link_id,
            s.created_at,
            s.updated_at
        FROM 
            Sessions s
        LEFT JOIN 
            TrainingDays td ON s.day_id = td.day_id
        LEFT JOIN 
            ExerciseTrainingDayLink etdl ON td.day_id = etdl.day_id
        LEFT JOIN 
            Exercises e ON etdl.exercise_id = e.exercise_id
        WHERE 
            s.day_id = $1
        "#,
        )
        .bind(&day_id);

        let rows = match query.fetch_all(&self.pool).await {
            Ok(rows) => rows,
            Err(err) => return Err(SessionError::Error(err.to_string())),
        };

        let mut sessions_map: HashMap<Uuid, SessionWithExercises> = HashMap::new();

        for row in rows {
            let session_id = row.session_id;
            let day_id = row.day_id;
            let in_progress = row.in_progress;
            let exercise_id = row.exercise_id;
            let exercise_name = row.exercise_name;
            let exercise_description = row.exercise_description;
            let link_id = row.link_id;
            let created_at = row.created_at;
            let updated_at = row.updated_at;

            let session = sessions_map
                .entry(session_id)
                .or_insert(SessionWithExercises {
                    session_id,
                    day_id,
                    in_progress,
                    exercises: Vec::new(),
                    created_at,
                    updated_at,
                });

            session.exercises.push(ExerciseWithLinkId {
                exercise_id: exercise_id.unwrap_or_default(),
                link_id,
                exercise_name,
                exercise_description,
                created_at,
                updated_at,
            });
        }

        let sessions: Vec<SessionWithExercises> = sessions_map.into_values().collect();

        for session in &sessions {
            if session.exercises.is_empty() {
                return Err(SessionError::NoExercisesFound);
            }
        }

        Ok(sessions)
    }

    async fn end_session(&self, session_id: &Uuid) -> SessionResult<()> {
        // Update the `in_progress` field of the session to false in the database
        sqlx::query(
            r#"
        UPDATE Sessions
        SET in_progress = FALSE
        WHERE session_id = $1
        "#,
        )
        .bind(session_id)
        .execute(&self.pool)
        .await
        .map_err(|e| SessionError::Error(e.to_string()))?;

        Ok(())
    }
}
