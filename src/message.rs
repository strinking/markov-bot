use super::schema::message;

#[derive(Queryable)]
#[derive(Insertable)]
#[table_name="message"]
pub struct Message {
    pub id: i64,
    pub content: String,
    pub guild_id: i64,
    pub author_id: i64,
    pub channel_id: i64,
}
