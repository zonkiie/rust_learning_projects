#[allow(unused_imports)]
use std::fmt;
#[allow(unused_imports)]
use async_std::*;
use futures::*;
use sea_orm::*;

use sea_orm::DatabaseConnection;
// use sea_orm::{entity::*, query::*};
// use serde::{Deserialize, Serialize};
// use std::env;

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

pub async fn do_query() -> String
{
    let mut retstr = String::new();
    retstr.push_str("Start\n");
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

    retstr.push_str("Connect\n");

    let conn: DatabaseConnection = Database::connect("sqlite::memory:").await.unwrap();
    let _ = setup::create_post_table(&conn).await;
    let _ = setup::create_user_table(&conn).await;

    let u = user::ActiveModel {
        firstname: Set("Hans".to_owned()),
        lastname: Set("Mustermann".to_owned()),
        username: Set("hamu".to_owned()),
        ..Default::default()
    };

    let user_insert_res = User::insert(u)
        .exec(&conn)
        .await
        .expect("could not insert user");
    
    let p = post::ActiveModel {
        title: Set("Titel".to_owned()),
        content: Set("The Content".to_owned()),
        author: Set(user_insert_res.last_insert_id as i32),
        ..Default::default()
    };

    let p_insert_res = Post::insert(p)
        .exec(&conn)
        .await
        .expect("could not insert post");

    let p_id = p_insert_res.last_insert_id;
    retstr.push_str(&(format!("Post Insert ID: {:?}\n", p_id)));
    
    let qu = User::find().one(&conn).await.unwrap();
    retstr.push_str(&(format!("Queried User: {:?}\n", qu)));
    let qp = Post::find().one(&conn).await.unwrap();
    retstr.push_str(&(format!("Queried Post: {:?}\n", qp)));
    retstr
}

#[async_std::main]
async fn main() {
    println!("Main");
    //let mut o:String = String::new();
    let o = async 
    {
        do_query().await
    };
    let values = executor::block_on(o);
    println!("Output: {}", values);

}
