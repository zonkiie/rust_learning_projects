use sea_orm::sea_query::{ColumnDef, TableCreateStatement};
use sea_orm::{error::*, sea_query, DbConn, ExecResult, ConnectionTrait};

#[allow(unused_imports)]
use crate::entities::*;


async fn create_table(db: &DbConn, stmt: &TableCreateStatement) -> Result<ExecResult, DbErr> {
    let builder = db.get_database_backend();
    db.execute(builder.build(stmt)).await
}

pub async fn create_user_table(db: &DbConn) -> Result<ExecResult, DbErr> {
    let stmt = sea_query::Table::create()
        .table(super::user::Entity)
        .if_not_exists()
        .col(
            ColumnDef::new(super::user::Column::Id)
                .integer()
                .not_null()
                .auto_increment()
                .primary_key(),
        )
        .col(
            ColumnDef::new(super::user::Column::Firstname)
                .string()
                .not_null()
        )
        .col(
            ColumnDef::new(super::user::Column::Lastname)
                .string()
                .not_null()
        )
        .col(
            ColumnDef::new(super::user::Column::Username)
                .string()
                .not_null(),
        )
        .to_owned();

    create_table(db, &stmt).await
}

pub async fn create_post_table(db: &DbConn) -> Result<ExecResult, DbErr> {
    let stmt = sea_query::Table::create()
        .table(super::post::Entity)
        .if_not_exists()
        .col(
            ColumnDef::new(super::post::Column::Id)
                .integer()
                .not_null()
                .auto_increment()
                .primary_key(),
        )
        .col(
            ColumnDef::new(super::post::Column::Author)
                .integer()
                .not_null()
        )
        .col(
            ColumnDef::new(super::post::Column::Title)
                .string()
                .not_null()
        )
        .col(
            ColumnDef::new(super::post::Column::Content)
                .string()
         )
        .to_owned();

    create_table(db, &stmt).await
}

pub async fn create_book_table(db: &DbConn) -> Result<ExecResult, DbErr> {
    let stmt = sea_query::Table::create()
        .table(super::book::Entity)
        .if_not_exists()
        .col(
            ColumnDef::new(super::book::Column::Id)
                .integer()
                .not_null()
                .auto_increment()
                .primary_key(),
        )
        .col(
            ColumnDef::new(super::book::Column::Author)
                .integer()
                .not_null()
        )
        .col(
            ColumnDef::new(super::book::Column::Title)
                .string()
                .not_null()
        )
        .col(
            ColumnDef::new(super::book::Column::BookNo)
                .integer()
         )
        .to_owned();

    create_table(db, &stmt).await
}
