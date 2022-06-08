use diesel::{r2d2::ConnectionManager, PgConnection};
use r2d2::PooledConnection;

#[macro_use]
extern crate diesel; 

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type Connection = PooledConnection<ConnectionManager<PgConnection>>;

pub mod handlers;
pub mod models;
pub mod schema;
pub mod db;
pub mod errors;