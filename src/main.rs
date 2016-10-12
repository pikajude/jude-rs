extern crate iron;
extern crate logger;
#[macro_use] extern crate router;
extern crate chrono;
extern crate rustache;
extern crate mount;
extern crate staticfile;
extern crate pulldown_cmark;
extern crate syntect;

extern crate r2d2;
extern crate r2d2_postgres;
extern crate postgres;

use iron::prelude::*;
use logger::Logger;
// use router::*;

use r2d2_postgres::{SslMode, PostgresConnectionManager};

use iron::typemap::Key;
use staticfile::Static;
use std::path::Path;
use mount::Mount;

mod highlighting;
mod pages;
mod db;

struct Auth;

impl Key for Auth {
    type Value = String;
}

// impl BeforeMiddleware for Auth {
//     fn before(&self, req: &mut Request) -> IronResult<()> {
//         let username = try!(req.get_session("user").map_err(|e| {
//             println!("{:?}", e);
//             IronError::new(e, (Status::InternalServerError, "Internal server error"))
//         }));
//         req.extensions.insert::<Auth>(username);
//         Ok(())
//     }
// }

fn main() {
    let router = router!(index: get "/" => pages::home::handle,
                         single: get "/r/:slug" => pages::single::handle,
                         static: get "/s/:file" => pages::home::handle // not used
                         );

    let config = r2d2::Config::default();
    let manager = match PostgresConnectionManager::new("postgres://jude@localhost", SslMode::None) {
        Ok(man) => man,
        Err(e) => panic!("Error connecting to jude@localhost: {}", e)
    };
    let pool = r2d2::Pool::new(config, manager).unwrap();

    let mut sql_chain = Chain::new(router);
    sql_chain.link_before(db::SqlPool { pool: pool });
    // sql_chain.link_before(Auth);

    let mut mnt = Mount::new();
    mnt.mount("/s/", Static::new(Path::new("static")));
    mnt.mount("/", sql_chain);

    let mut log_chain = Chain::new(mnt);
    log_chain.link(Logger::new(None));

    Iron::new(log_chain).http("[::]:3000").unwrap();
}
