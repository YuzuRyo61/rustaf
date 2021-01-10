use serde::{Serialize, Deserialize};
use super::schema::messages;

#[derive(Queryable, Serialize)]
pub struct Messages {
    pub id: i32,
    pub msg: String
}

#[derive(Deserialize, Insertable)]
#[table_name="messages"]
pub struct CreateMessages {
    pub msg: String
}
