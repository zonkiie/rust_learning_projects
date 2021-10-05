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


fn main() {
    println!("Hello, world!");
}
