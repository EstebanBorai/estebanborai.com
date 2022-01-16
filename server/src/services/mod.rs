pub mod github;
pub mod notes;

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use std::sync::Arc;

use crate::config::Config;

use self::github::GitHubService;
use self::notes::NotesService;

pub type PgConnPool = Pool<ConnectionManager<PgConnection>>;

pub struct Services {
    pub dbconn_pool: Arc<PgConnPool>,
    pub github_service: Arc<GitHubService>,
    pub notes_service: Arc<NotesService>,
}

impl Services {
    pub async fn new(config: &Config) -> Self {
        let dbconn_pool = Services::make_connection_pool(config);
        let dbconn_pool = Arc::new(dbconn_pool);
        let github_service = Arc::new(GitHubService::new(
            &config.github_api_user,
            &config.github_api_token,
        ));
        let notes_service = Arc::new(NotesService::new(
            Arc::clone(&github_service),
            Arc::clone(&dbconn_pool),
        ));

        Services {
            dbconn_pool,
            github_service,
            notes_service,
        }
    }

    fn make_connection_pool(config: &Config) -> PgConnPool {
        let manager = ConnectionManager::<PgConnection>::new::<&String>(&config.database_url);

        Pool::new(manager).expect("Failed to initialize database connection pool")
    }
}
