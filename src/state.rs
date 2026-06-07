use color_eyre::Result;
use std::sync::Arc;

use tokio::sync::Mutex;

use crate::database::Database;

pub(crate) struct AppState {
    database: Database,
}

impl AppState {
    pub async fn new_shared() -> Result<SharedState> {
        let db = Database::new(&std::env::var("DATABASE_URL")?).await?;

        let app_state = AppState { database: db };
        Ok(Arc::new(Mutex::new(app_state)))
    }

    pub fn database(&self) -> &Database {
        &self.database
    }

    pub fn database_mut(&mut self) -> &mut Database {
        &mut self.database
    }
}

pub type SharedState = Arc<Mutex<AppState>>;
