pub mod github;
pub mod notes;

use std::sync::Arc;

use self::github::GitHubService;
use self::notes::NotesService;

pub struct Services {
    pub github_service: Arc<GitHubService>,
    pub notes_service: Arc<NotesService>,
}

impl Services {
    pub fn new() -> Self {
        let github_service = Arc::new(GitHubService::new());
        let notes_service = Arc::new(NotesService::new(Arc::clone(&github_service)));

        Services {
            github_service,
            notes_service,
        }
    }
}
