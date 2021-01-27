use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

#[get("/<id>")]
pub fn get_user_by_id(id: usize) -> Json<User> {
    Json(User {
        id,
        name: String::from("lilei"),
        age: 18,
    })
}

#[get("/")]
pub fn index() -> &'static str {
    "get user"
}

// 用户
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: usize,
    pub name: String,
    pub age: u8,
}