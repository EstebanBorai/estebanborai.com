pub mod github;
pub mod notes;

use sqlx::postgres::PgPool;
use std::env;
use std::sync::Arc;

use crate::config::Config;

use self::github::GitHubService;
use self::notes::NotesService;

pub struct Services {
    pub database: Arc<PgPool>,
    pub github_service: Arc<GitHubService>,
    pub notes_service: Arc<NotesService>,
}

impl Services {
    pub async fn new(config: &Config) -> Self {
        let database = PgPool::connect(&config.database_url)
            .await
            .expect("Failed to create a database connection pool instance");
        let database = Arc::new(database);
        let github_service = Arc::new(GitHubService::new());
        let notes_service = Arc::new(NotesService::new(
            Arc::clone(&github_service),
            Arc::clone(&database),
        ));

        Services {
            database,
            github_service,
            notes_service,
        }
    }
}
