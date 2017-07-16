use super::schema::nodes;

build_model!(Node; NewNode; "nodes" => {
    key: String,
    value: String
});
