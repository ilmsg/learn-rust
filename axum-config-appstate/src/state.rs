use crate::config::Settings;
use crate::repository::UserRepostiroy;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Settings>,
    pub user_repo: UserRepostiroy,
}

impl AppState {
    pub fn new(settings: Settings) -> Self {
        let db_config = Arc::new(settings.db.clone());
        let user_repo = UserRepostiroy::new(db_config);

        Self {
            config: Arc::new(settings),
            user_repo,
        }
    }
}
