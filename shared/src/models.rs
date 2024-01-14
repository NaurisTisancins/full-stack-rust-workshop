use serde::{Deserialize, Serialize};

// #[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
// #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
// pub struct Film {
//     pub id: uuid::Uuid, // we will be using uuids as ids
//     pub title: String,
//     pub director: String,
//     #[cfg_attr(feature = "backend", sqlx(try_from = "i16"))]
//     pub year: u16, // only positive numbers
//     pub poster: String, // we will use the url of the poster here
//     pub created_at: Option<chrono::DateTime<chrono::Utc>>,
//     pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
// }

// #[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
// #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
// pub struct CreateFilm {
//     pub title: String,
//     pub director: String,
//     #[cfg_attr(feature = "backend", sqlx(try_from = "i16"))]
//     pub year: u16,
//     pub poster: String,
// }

// Routines model
#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Routine {
    pub routine_id: uuid::Uuid, // we will be using uuids as ids
    pub name: String,
    pub description: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct CreateRoutine {
    pub name: String,
    pub description: String,
}
