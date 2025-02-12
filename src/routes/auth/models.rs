use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub google_id: Option<String>,
    pub password_hash: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32, // ID пользователя
    pub roles: Vec<String>, // Роли пользователя
    pub exp: usize, // Время истечения JWT
}