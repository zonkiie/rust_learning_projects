#![allow(unused_imports)]

extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate serde_value;
extern crate quick_xml;

use std::*;
use std::fmt::*;
use std::error::Error;
use itertools::*;
use serde::*;
use serde::de::*;
use serde_value::*;
use quick_xml::de::*;
use quick_xml::se::*;

use schema::*;
use crate::schema::*;
use diesel::*;
use diesel::prelude::*;
use models::*;
use diesel_demo2::*;

fn main() -> std::result::Result<(), Box<dyn Error>> {
    use schema::users::dsl::*;
    use schema::posts::dsl::*;

    let connection = establish_connection();
    //let userlist = users::table.load::<User>(&connection)?;
    let userlist = users.load::<User>(&connection)?;
    // variable name posts is not allowed because it's a type
	let userposts = Post::belonging_to(&userlist)
		.load::<Post>(&connection)?
		.grouped_by(&userlist);
	let data = userlist.into_iter().zip(userposts).collect::<Vec<_>>();
	// This is the way to serialize simple structs with serde
	//let str = to_string(&data).unwrap(); println!("Line: {}", str);
	let str = serde_json::to_string(&data)?;
	println!("Line: {}", str);
	// no semikolon when returning values
	
	// from: https://github.com/diesel-rs/diesel/issues/89
	//let data2 = users::table.left_outer_join(posts::table).load(&connection)
	let data2 = users.left_outer_join(posts).load(&connection)?
		//.group_by(|(user, post)| user)
		.group_by(|(users, posts)| users)
		//.map(|(user, posts)| posts.into_iter().filter_map(|p| p).collect())
		.map(|(users, posts)| posts.into_iter().filter_map(|p| p).collect())
		.collect();
	let str2 = serde_json::to_string(&data2)?;
	println!("Line: {}", str);
	
	
	Ok(())
}	
