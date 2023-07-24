use sqlx::FromRow;

#[derive(Debug, FromRow, serde::Serialize, serde::Deserialize, Clone)]
pub struct User {
    pub user_id: i32,
    pub username: String,
    pub password: String,
    pub handle: String,
    pub email: String,
    pub created_at: String,
    pub update_at: String,
}

#[derive(Debug, FromRow, serde::Serialize, serde::Deserialize, Clone)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub author_username: String,
    pub author_handle: String,
    pub author_id: i32,
    pub body: String,
    pub created_at: String,
    pub updated_at: String,
}
pub enum DatabaseActionError<T> {
    ReadError(Option<T>),
    WriteError(Option<T>),
    ExternalError(Option<T>),
}

pub enum UserError {
    GetError,
    CreateError,
    UpdateError,
    DeleteError,
}
