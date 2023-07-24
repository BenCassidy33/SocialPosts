use super::{
    models::{Post, User},
    traits::{GetPost, GetPosts, GetUser, GetUsers},
};

impl GetUser<Vec<User>> for Vec<User> {
    fn get_by_id(&self, id: i32) -> Result<Vec<User>, String> {
        for user in self.into_iter() {
            if user.user_id == id {
                return Ok(vec![user.clone().into()]);
            } else {
                continue;
            }
        }

        Err("User not found".into())
    }

    fn get_by_email(&self, email: String) -> Result<Vec<User>, String> {
        for user in self.into_iter() {
            if user.email == email {
                return Ok(vec![user.clone().into()]);
            } else {
                continue;
            }
        }

        Err("User not found".into())
    }

    fn get_by_handle(&self, handle: String) -> Result<Vec<User>, String> {
        for user in self.into_iter() {
            if user.handle == handle {
                return Ok(vec![user.clone().into()]);
            } else {
                continue;
            }
        }

        Err("User not found".into())
    }

    fn get_by_username(&self, username: String) -> Result<Vec<User>, String> {
        for user in self.into_iter() {
            if user.username == username {
                return Ok(vec![user.clone().into()]);
            } else {
                continue;
            }
        }

        Err("User not found".into())
    }
}

impl GetPost<Vec<Post>> for Vec<Post> {
    fn get_by_id(&self, id: i32) -> Result<Vec<Post>, String> {
        for post in self.into_iter() {
            if post.id == id {
                return Ok(vec![post.clone().into()]);
            } else {
                continue;
            }
        }

        Err("Post not found".into())
    }

    fn get_by_user_id(&self, user_id: i32) -> Result<Vec<Post>, String> {
        for post in self.into_iter() {
            if post.id == user_id {
                return Ok(vec![post.clone().into()]);
            } else {
                continue;
            }
        }

        Err("Post not found".into())
    }

    fn get_by_user_handle(&self, user_handle: String) -> Result<Vec<Post>, String> {
        for post in self.into_iter() {
            if post.author_handle == user_handle {
                return Ok(vec![post.clone().into()]);
            } else {
                continue;
            }
        }

        Err("Post not found".into())
    }
}
