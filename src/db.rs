extern crate r2d2_postgres;
extern crate r2d2;

use iron::typemap::Key;
use iron::prelude::*;
use iron::BeforeMiddleware;
use r2d2_postgres::PostgresConnectionManager;

pub struct SqlPool {
    pub pool: r2d2::Pool<PostgresConnectionManager>
}

impl Key for SqlPool {
    type Value = r2d2::Pool<PostgresConnectionManager>;
}

impl BeforeMiddleware for SqlPool {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        req.extensions.insert::<SqlPool>(self.pool.clone());
        Ok(())
    }
}
