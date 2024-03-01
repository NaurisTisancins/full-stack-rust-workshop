use std::cmp::Ordering;

use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
// use sqlx::{Decode, Postgres};
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
    pub day_name: String,
    pub in_progress: bool,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct SessionWithExercises {
    pub session_id: uuid::Uuid,
    pub day_id: uuid::Uuid,
    pub day_name: String,
    pub in_progress: bool,
    pub exercises: Vec<ExerciseWithLinkId>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct SessionPerformance {
    pub session_id: uuid::Uuid,
    pub exercise_id: uuid::Uuid,
    pub exercise_name: String,
    pub sets: Vec<SetPerformance>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl SessionPerformance {
    // Custom constructor to create SessionPerformance instances with an empty sets vector
    pub fn new(session_id: uuid::Uuid, exercise_id: uuid::Uuid, exercise_name: String) -> Self {
        Self {
            session_id,
            exercise_id,
            exercise_name,
            sets: Vec::new(), // Initialize sets vector as empty
            created_at: None,
            updated_at: None,
        }
    }
}

#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct SetPerformance {
    pub performance_id: uuid::Uuid,
    pub weight: f32,
    pub reps: i16,
    pub set_number: i16,
    pub rir: Option<i16>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl Eq for SetPerformance {}

impl Ord for SetPerformance {
    fn cmp(&self, other: &Self) -> Ordering {
        self.set_number.cmp(&other.set_number)
    }
}

#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct SetPerformancePayload {
    pub weight: f32,
    pub reps: i16,
    pub set_number: i16,
    pub rir: Option<i16>,
}

impl Eq for SetPerformancePayload {}

impl Ord for SetPerformancePayload {
    fn cmp(&self, other: &Self) -> Ordering {
        self.set_number.cmp(&other.set_number)
    }
}

#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct SessionWithExercisePerformance {
    pub session_id: Uuid,
    pub day_id: Uuid,
    pub day_name: String,
    pub in_progress: bool,
    pub exercises: Vec<ExerciseWithLinkId>,
    pub performance: Vec<SessionPerformance>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct SessionsWithExercisesQuery {
    pub session_id: Uuid,
    pub day_id: Uuid,
    pub day_name: String,
    pub in_progress: bool,
    pub exercise_id: Option<Uuid>,
    pub exercise_name: String,
    pub exercise_description: String,
    pub link_id: Uuid,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}
