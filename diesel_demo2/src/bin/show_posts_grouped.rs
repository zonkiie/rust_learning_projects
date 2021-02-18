#![allow(unused_imports)]

extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate serde_value;
extern crate quick_xml;

use std::fmt::*;
use serde::*;
use serde::de::*;
use serde_value::*;
use quick_xml::de::*;
use quick_xml::se::*;

use schema::*;
use diesel::*;
use diesel::prelude::*;
use diesel_demo2::models::*;
use diesel_demo2::*;

fn main() {
    use diesel_demo2::schema::posts::dsl::*;

    let connection = establish_connection();
    let users = users::table.load::<User>(&connection)?;
	let posts = Post::belonging_to(&users)
		.load::<Post>(&connection)?
		.grouped_by(&users);
	let data = users.into_iter().zip(posts).collect::<Vec<_>>();
	let str = to_string(&data).unwrap();
	println!("Line: {}", str);
}	
