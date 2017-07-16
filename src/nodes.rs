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
        let found = nodes.filter(nodes::key.eq(&node.key)).first::<Node>(conn).ok();
        if found.is_some() {
            found.unwrap()
        } else {
            diesel::insert(&node).into(nodes::table)
                .get_result(conn)
                .expect("Could not insert new Node")
        }
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

    pub fn update_key(conn: &PgConnection, key_to_update: &str, new_value: &str) -> Node {
        diesel::update(nodes.filter(nodes::key.eq(key_to_update)))
                .set(value.eq(new_value))
                .get_result::<Node>(conn)
                .expect("Could not update post")
    }

    pub fn delete_key(conn: &PgConnection, key_to_delete: &str) -> i32 {
        let result = diesel::delete(nodes.filter(nodes::key.eq(key_to_delete))).execute(conn).ok();
        if result.is_some() {
            return 1;
        }
        return 0;
    }
}
