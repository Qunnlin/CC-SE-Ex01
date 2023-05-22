#![allow(unused_doc_comments)]
#![allow(dead_code)]
/// This file contains the database connection and pool used by both Services
///
/// The database connection is established using the [establish_connection] function
/// Alternatively, a connection pool can be created using the [create_pool] function
///
/// The database schema is defined in [schema.rs](../schema.rs.html)

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PoolError};

use dotenv::dotenv;
use std::env;
use diesel::{Connection};

///
/// Establishes a connection to the database
///
/// Returns a [PgConnection]
pub fn establish_connection() -> PgConnection {
    /// Load the .env file
    dotenv().ok();
    /// Get the database URL from the environment
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    /// Establish a connection to the database
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub fn create_pool() -> Result<DbPool, PoolError> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager)
}

