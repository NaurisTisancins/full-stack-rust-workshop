use std::ptr::null;

use super::{RoutineResult, RoutinesRepository};
use shared::models::{CreateRoutine, Routine};
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
}
