/// ! Common database utils for wabper
///

#[macro_use]
extern crate diesel;
pub mod models;
pub mod schema;
pub mod structures;
use diesel::prelude::*;
use wabper_common::Error;

pub fn db_get_connection(
) -> Result<diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<PgConnection>>, Error> {
    let db_url = std::env::var("DATABASE_URL")?;
    let manager: diesel::r2d2::ConnectionManager<PgConnection> =
        diesel::r2d2::ConnectionManager::new(&db_url);

    Ok(diesel::r2d2::Pool::new(manager)?)
}