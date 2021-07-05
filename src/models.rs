use super::schema::cats;
use serde::{Deserialize, Serialize};

#[derive(queryable, Serialize)]
pub struct Cat {
    pub id: i32,
    pub name: String,
    pub image_path: String,
}
