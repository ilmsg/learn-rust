mod config;
mod handler;
mod repository;
mod state;

use axum::{Router, routing::get};
use config::Settings;
use state::AppState;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let settings = Settings::new().expect("โหลด Config ไม่สำเร็จ");
    let port = settings.app.port;

    let app_state = AppState::new(settings);

    let app = Router::new()
        .route("/user", get(handler::get_user_handler))
        .with_state(app_state);

    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("Server กำลังรันที่ http://{}", addr);

    axum::serve(listener, app).await.unwrap();

    //กำลังค้นหา User 42 จาก DB: sqlite:://user.db (Max Conn: 5) | ความยาว JWT Secret: 12
}
