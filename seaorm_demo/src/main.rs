use sea_orm::*;

use listenfd::ListenFd;
use sea_orm::DatabaseConnection;
use sea_orm::{entity::*, query::*};
use serde::{Deserialize, Serialize};
use std::env;
use tera::Tera;

mod user;
mod post;
pub use user::Entity as User;
pub use post::Entity as Post;
mod setup;


fn main() {
    println!("Hello, world!");
}
