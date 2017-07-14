use super::schema::nodes;

#[table_name="nodes"]
#[derive(Queryable, Serialize, Deserialize, Insertable, Debug, Clone)]
pub struct Node {
    pub id: i32,
    pub key: String,
    pub value: String,
}
