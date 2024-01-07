pub use postgres_routines_repository::PostgresRoutinesRepository;
use shared::models::{CreateRoutine, Routine};
use uuid::Uuid;

pub type RoutineError = String;
pub type RoutineResult<T> = Result<T, RoutineError>;

#[async_trait::async_trait]
pub trait RoutinesRepository: Send + Sync + 'static {
    async fn get_routines(&self) -> RoutineResult<Vec<Routine>>;
    // async fn get_routine(&self, id: &Uuid) -> RoutineResult<Routine>;
    async fn create_routine(&self, id: &CreateRoutine) -> RoutineResult<Routine>;
    // async fn update_routine(&self, id: &Routine) -> RoutineResult<Routine>;
    // async fn delete_routine(&self, id: &Uuid) -> RoutineResult<Uuid>;
}

mod postgres_routines_repository;
