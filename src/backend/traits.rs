use super::models::{Post, User};

pub trait GetUser<T> {
    fn get_by_id(&self, id: i32) -> Result<T, String>;
    fn get_by_username(&self, username: String) -> Result<T, String>;
    fn get_by_email(&self, email: String) -> Result<T, String>;
    fn get_by_handle(&self, handle: String) -> Result<T, String>;
}

pub trait GetUsers<T> {
    fn get_all(&self) -> Result<T, String>;
    fn get_by_created_at(&self, created_at: String) -> Result<T, String>;
}

pub trait GetPost<T> {
    fn get_by_id(&self, id: i32) -> Result<T, String>;
    fn get_by_title(&self, title: String) -> Result<T, String>;
    fn get_by_body(&self, body: String) -> Result<T, String>;
    fn get_by_author_username(&self, author_username: String) -> Result<T, String>;
    fn get_by_author_handle(&self, author_handle: String) -> Result<T, String>;
    fn get_by_author_id(&self, author_id: i32) -> Result<T, String>;
}

pub trait GetPosts<T> {
    fn get_all(&self) -> Result<T, String>;
    fn get_by_created_at(&self, created_at: String) -> Result<T, String>;
    fn get_by_user_id(&self, user_id: i32) -> Result<T, String>;
    fn get_by_user_handle(&self, user_handle: String) -> Result<T, String>;
    fn get_by_title(&self, title: String) -> Result<T, String>;
    fn get_by_body(&self, body: String) -> Result<T, String>;
}

//     pub id: i32,
//     pub title: String,
//     pub author_username: String,
//     pub author_handle: String,
//     pub author_id: i32,
//     pub body: String,
//     pub created_at: String,
//     pub updated_at: String,
