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
// use iron::status;
// use router::*;

use r2d2_postgres::{SslMode, PostgresConnectionManager};

use staticfile::Static;
use std::path::Path;
use mount::Mount;

mod highlighting;
mod pages;
mod db;

fn main() {
    let router = router!(index: get "/" => pages::home::handle,
                         single: get "/r/:slug" => pages::single::handle,
                         // never hit, only used for link generation
                         static: get "/s/:file" => pages::home::handle);

    let mut mnt = Mount::new();
    mnt.mount("/s/", Static::new(Path::new("static")));
    mnt.mount("/", router);

    let config = r2d2::Config::default();
    let manager = PostgresConnectionManager::new("postgres://jude@localhost", SslMode::None).unwrap();
    let pool = r2d2::Pool::new(config, manager).unwrap();

    let mut chain = Chain::new(mnt);
    chain.link_before(db::SqlPool { pool: pool });

    chain.link(Logger::new(None));

    Iron::new(chain).http("localhost:3000").unwrap();
}
