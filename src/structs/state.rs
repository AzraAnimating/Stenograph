
use deadpool_postgres::{Manager, Object, Pool, PoolError};


#[derive(Clone)]
pub struct AppState {
    pub database: Pool
}

impl AppState {
    pub async fn get_database(&self) ->  Result<Object, PoolError> {
        self.database.clone().get().await
    }
}
