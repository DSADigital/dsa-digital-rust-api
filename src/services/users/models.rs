use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct UserModel {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub password: Option<String>,
    pub role: String,
    pub token: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
}
