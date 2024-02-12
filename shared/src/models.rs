use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, Decode, Postgres};
use uuid::Uuid;

// Routines model
#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Routine {
    pub routine_id: uuid::Uuid, // we will be using uuids as ids
    pub name: String,
    pub description: String,
    pub is_active: bool,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct CreateRoutine {
    pub name: String,
    pub description: String,
    pub is_active: bool,
}

#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct TrainingDay {
    pub day_id: uuid::Uuid, // we will be using uuids as ids
    pub routine_id: uuid::Uuid,
    pub day_name: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct CreateTrainingDay {
    pub routine_id: uuid::Uuid,
    pub day_name: String,
}

#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Exercise {
    pub exercise_id: uuid::Uuid, // we will be using uuids as ids
    pub exercise_name: String,
    pub exercise_description: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct SearchQuery {
    pub name: String,
}

#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct CreateExercise {
    pub exercise_name: String,
    pub exercise_description: String,
}

#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct ExerciseToTrainingDay {
    pub link_id: uuid::Uuid,
    pub exercise_id: uuid::Uuid,
    pub day_id: uuid::Uuid,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct ExerciseWithLinkId {
    pub exercise_id: uuid::Uuid, // we will be using uuids as ids
    pub link_id: uuid::Uuid,
    pub exercise_name: String,
    pub exercise_description: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct TrainingDayWithExercises {
    pub day_id: uuid::Uuid, // we will be using uuids as ids
    pub routine_id: uuid::Uuid,
    pub day_name: String,
    pub exercises: Option<Vec<ExerciseWithLinkId>>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct TrainingDayWithExercisesQuery {
    pub day_id: uuid::Uuid,
    pub routine_id: uuid::Uuid,
    pub link_id: Option<uuid::Uuid>,     // Make link_id optional
    pub exercise_id: Option<uuid::Uuid>, // Make exercise_id optional
    pub day_name: String,
    pub exercise_name: Option<String>, // Make exercise_name optional
    pub exercise_description: Option<String>, // Make exercise_description optional
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Session {
    pub session_id: uuid::Uuid,
    pub day_id: uuid::Uuid,
    pub in_progress: bool,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct SessionWithExercises {
    pub session_id: uuid::Uuid,
    pub day_id: uuid::Uuid,
    pub in_progress: bool,
    pub exercises: Vec<ExerciseWithLinkId>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct SessionExercisePerformance {
    pub performance_id: uuid::Uuid,
    pub session_id: uuid::Uuid,
    pub exercise_id: uuid::Uuid,
    pub weight: i64,
    pub reps: i16,
    pub set: i16,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct SessionsWithExercisesQuery {
    pub session_id: Uuid,
    pub day_id: Uuid,
    pub in_progress: bool,
    pub exercise_id: Option<Uuid>,
    pub exercise_name: String,
    pub exercise_description: String,
    pub link_id: Uuid,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}
