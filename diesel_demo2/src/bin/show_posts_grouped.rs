#![allow(unused_imports)]

extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate serde_value;
extern crate quick_xml;

use std::*;
use std::fmt::*;
use std::error::Error;
use serde::*;
use serde::de::*;
use serde_value::*;
use quick_xml::de::*;
use quick_xml::se::*;

use schema::{users, posts};
use diesel::*;
use diesel::prelude::*;
use diesel_demo2::models::*;
use diesel_demo2::*;


//fn main() -> std::io::Result<(), std::io::Error> {
fn main() -> std::result::Result<(), Box<dyn Error>> {
    use diesel_demo2::schema::posts::dsl::*;

    let connection = establish_connection();
    let users = users::table.load::<User>(&connection)?;
    // variable name posts is not allowed because it's a type
	let userposts = Post::belonging_to(&users)
		.load::<Post>(&connection)?
		.grouped_by(&users);
	let data = users.into_iter().zip(userposts).collect::<Vec<_>>();
	println!("Data: {:?}", data);
	// let str = to_string(&data).unwrap(); println!("Line: {}", str);
	Ok(())
}	
