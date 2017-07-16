use std::env;
use dotenv::dotenv;
use diesel::prelude::*;
use diesel::pg::PgConnection;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

macro_rules! build_model {
    ( $struct_name:ident; $new_struct_name:ident; $table:expr => {
        $( $attr_name:ident : $attr_type:ty ),*
    }) => {
        #[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
        pub struct $struct_name {
            pub id: i32,
            $( pub $attr_name : $attr_type ),*
        }

        #[table_name=$table]
        #[derive(Serialize, Deserialize, Insertable, Debug, Clone)]
        pub struct $new_struct_name {
            $( pub $attr_name : $attr_type ),*
        }
    }
}
