use serde::Deserialize;
#[derive(Deserialize, Debug, Clone)]
pub struct UserData {
    pub user_id: i32,
    pub id: i32,
    pub title: String,
    pub completed: bool,
}

