use sea_orm::DatabaseConnection;

/// 構造体: ステート
#[derive(Debug, Clone)]
pub struct AppState {
    pub conn: DatabaseConnection,
}
