use deadpool_postgres::Pool;

#[derive(Clone)]
pub struct AppState {
    pub database: Pool
}
