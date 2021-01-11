use super::schema::messages;

#[derive(Queryable, Serialize)]
pub struct Messages {
    pub id: i32,
    pub msg: Option<String>
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "messages"]
pub struct CreateMessages {
    pub msg: Option<String>
}
