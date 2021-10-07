use sea_orm::*;

use listenfd::ListenFd;
use sea_orm::DatabaseConnection;
use sea_orm::{entity::*, query::*};
use serde::{Deserialize, Serialize};
use std::env;
use tera::Tera;

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

pub async fn do_query()
{
    // get env vars
    dotenv::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{}:{}", host, port);

    // create post table if not exists
    let conn = sea_orm::Database::connect(&db_url).await.unwrap();
    let _ = setup::create_post_table(&conn).await;
    let _ = setup::create_user_table(&conn).await;
}

fn main() {
    do_query();
}
