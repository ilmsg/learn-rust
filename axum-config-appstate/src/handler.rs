use crate::state::AppState;
use axum::{extract::State, response::IntoResponse};

pub async fn get_user_handler(State(state): State<AppState>) -> impl IntoResponse {
    let result = state.user_repo.find_user(42).await;
    let secret_len = state.config.app.jwt_secret.len();
    format!("{} | ความยาว JWT Secret: {}", result, secret_len)
}
