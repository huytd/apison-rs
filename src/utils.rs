use std::env;
use dotenv::dotenv;
use diesel::prelude::*;
use diesel::pg::PgConnection;

use rocket::Outcome;
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};

pub struct AuthToken(String);

impl<'a, 'r> FromRequest<'a, 'r> for AuthToken {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<AuthToken, ()> {
        let keys: Vec<_> = request.headers().get("AuthToken").collect();
        if keys.len() != 1 {
            return Outcome::Failure((Status::BadRequest, ()));
        }
        let key = keys[0];

        return Outcome::Success(AuthToken(key.to_string()));
    }
}

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


