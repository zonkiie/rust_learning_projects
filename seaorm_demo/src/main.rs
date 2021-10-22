#[allow(unused_imports)]
use std::fmt;
#[allow(unused_imports)]
use async_std::*;
use futures::*;
use sea_orm::*;

#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde::de::{DeserializeOwned};
#[allow(unused_imports)]
use quick_xml::de::{from_str, DeError};
#[allow(unused_imports)]
use quick_xml::se::to_string;


#[allow(unused_imports)]
use std::env;

mod entities;
use crate::entities::*;

pub use entities::user::Entity as User;
pub use entities::post::Entity as Post;
mod setup;


#[derive(Debug, Clone)]
struct AppState {
    templates: tera::Tera,
    conn: DatabaseConnection,
}

pub async fn setup_db() -> DatabaseConnection
{
    // get env vars
    dotenv::dotenv().ok();
    /*
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{}:{}", host, port);

    // create post table if not exists
    let conn = sea_orm::Database::connect(&db_url).await.unwrap();
    */

    let conn: DatabaseConnection = Database::connect("sqlite::memory:").await.unwrap();
    let _ = setup::create_post_table(&conn).await;
    let _ = setup::create_user_table(&conn).await;
    conn
}

pub async fn insert_db(conn: &DatabaseConnection) -> bool
{
    let u = user::ActiveModel {
        firstname: Set("Hans".to_owned()),
        lastname: Set("Mustermann".to_owned()),
        username: Set("hamu".to_owned()),
        ..Default::default()
    };

    let user_insert_res = User::insert(u)
        .exec(conn)
        .await
        .expect("could not insert user");
    
    let u2 = user::ActiveModel {
        firstname: Set("Helga".to_owned()),
        lastname: Set("Musterfrau".to_owned()),
        username: Set("hemu".to_owned()),
        ..Default::default()
    };

#[allow(unused_variables)]
    let user_insert_res2 = User::insert(u2)
    .exec(conn)
    .await
    .expect("could not insert user");


    let p = post::ActiveModel {
        title: Set("Titel".to_owned()),
        content: Set("The Content".to_owned()),
        author: Set(user_insert_res.last_insert_id as i32),
        ..Default::default()
    };

    #[allow(unused_variables)]
    let p_insert_res = Post::insert(p)
        .exec(conn)
        .await
        .expect("could not insert post");

    //let p_id = p_insert_res.last_insert_id;
    true
}

pub async fn do_query() -> String
{
    let mut retstr = String::new();
    let conn: DatabaseConnection = setup_db().await;
    insert_db(&conn).await;
    
    let qu = User::find()
        .find_with_related(Post)
        .all(&conn)
        .await
        .unwrap();
    //retstr.push_str(&to_string(&qu).unwrap());
    retstr.push_str(&(format!("Queried User with post: {:#?}\n", qu)));
    retstr
}

#[async_std::main]
async fn main() {
    //let mut o:String = String::new();
    let o = async 
    {
        do_query().await
    };
    let values = executor::block_on(o);
    println!("Output: {}", values);

}
