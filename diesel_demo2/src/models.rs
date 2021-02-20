//use super::schema::posts;
//use schema::posts;
//use schema::*;
use super::*;
use crate::schema::*;
use schema::*;
use diesel::*;
use diesel::prelude::*;
use diesel::sql_types::*;

#[derive(Identifiable, Queryable, Associations, PartialEq, Serialize, Deserialize, Debug)]
#[belongs_to(User, foreign_key="owner_id")]
#[table_name = "posts"]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
    //pub ctime: Timestamp,
    pub ctime: String,
    pub owner_id: i32
}

#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
    pub owner_id: &'a i32,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Serialize, Deserialize, Debug)]
#[table_name = "users"]
pub struct User {
	pub id: i32,
	pub first_name: String,
	pub last_name: String, 
	//pub ctime: Timestamp,
	pub ctime: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
}

