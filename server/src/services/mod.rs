pub mod github;
pub mod notes;

use sqlx::postgres::PgPool;
use std::env;
use std::sync::Arc;

use self::github::GitHubService;
use self::notes::NotesService;

pub struct Services {
    pub database: Arc<PgPool>,
    pub github_service: Arc<GitHubService>,
    pub notes_service: Arc<NotesService>,
}

impl Services {
    pub async fn new() -> Self {
        let database = PgPool::connect(
            &env::var("DATABASE_URL").expect("Missing DATABASE_URL environment variable"),
        )
        .await
        .expect("Failed to create a database connection pool instance");
        let database = Arc::new(database);
        let github_service = Arc::new(GitHubService::new());
        let notes_service = Arc::new(NotesService::new(Arc::clone(&github_service)));

        Services {
            database,
            github_service,
            notes_service,
        }
    }
}
