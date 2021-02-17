/*
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
*/

#![allow(unused_imports)]
#![recursion_limit="256"]

#[macro_use] extern crate diesel;
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate serde_value;
extern crate quick_xml;

pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use std::fmt::Debug;
use serde::{Serialize, Deserialize};
use serde::de::DeserializeOwned;
use serde_value::Value;
use quick_xml::de::{from_str, DeError};
use quick_xml::se::to_string;

use models::*;
use self::models::*;

pub fn establish_connection() -> SqliteConnection {
	dotenv().ok();

	let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
	SqliteConnection::establish(&database_url)
		.unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_post(conn: &SqliteConnection, title: &str, body: &str, owner_id: &i32) -> usize {
	//use crate::schema::posts;
	use crate::schema::*;

	let new_post = NewPost { title, body, owner_id };

	diesel::insert_into(posts::table)
		.values(&new_post)
		.execute(conn)
		.expect("Error saving new post")
}
