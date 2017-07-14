use models::{Node};

use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use schema::nodes;
use schema::nodes::dsl::*;

impl Node {
    pub fn new(conn: &PgConnection, node_key: &str, node_value: &str) -> Node {
        let next_id = (nodes.count().get_result(conn).unwrap_or(0) + 1) as i32;
        Node {
            id: next_id,
            key: node_key.to_string(),
            value: node_value.to_string()
        }
    }

    pub fn create(conn: &PgConnection, node: Node) -> Node {
        diesel::insert(&node).into(nodes::table)
            .get_result(conn)
            .expect("Could not insert new Node")
    }

    pub fn get_all_nodes(conn: &PgConnection) -> Vec<Node> {
        let total = nodes.load::<Node>(conn).ok();
        if total.is_some() {
            let result = total.unwrap();
            return result;
        }
        vec!()
    }

    pub fn get_by_key(conn: &PgConnection, key_to_find: &str) -> Option<Node> {
        nodes.filter(nodes::key.eq(key_to_find)).first::<Node>(conn).ok()
    }
}
