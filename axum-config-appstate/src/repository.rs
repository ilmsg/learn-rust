use crate::config::DbConfig;
use std::sync::Arc;

#[derive(Clone)]
pub struct UserRepostiroy {
    db_config: Arc<DbConfig>,
}

impl UserRepostiroy {
    pub fn new(db_config: Arc<DbConfig>) -> Self {
        Self { db_config }
    }

    pub async fn find_user(&self, user_id: u32) -> String {
        format!(
            "กำลังค้นหา User {} จาก DB: {} (Max Conn: {})",
            user_id, self.db_config.url, self.db_config.max_connections
        )
    }
}
