use models::{Node};

use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use schema::nodes;
use schema::nodes::dsl::*;

impl Node {
    fn new(conn: &PgConnection, node_key: String, node_value: String) -> Node {
        let next_id = (nodes.count().get_result(conn).unwrap_or(0) + 1) as i32;
        Node {
            id: next_id,
            key: node_key,
            value: node_value
        }
    }

    fn create(conn: &PgConnection, node: Node) -> Node {
        diesel::insert(&node).into(nodes::table)
            .get_result(conn)
            .expect("Could not insert new Node")
    }
}
