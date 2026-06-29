pub mod gen;

use diesel_async::{
    pooled_connection::{deadpool::Pool, AsyncDieselConnectionManager},
    AsyncPgConnection,
};

pub fn create_pool(database_url: &str) -> Result<Pool<AsyncPgConnection>, deadpool::managed::BuildError> {
    let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(database_url);
    Pool::builder(config).build()
}
