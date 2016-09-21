use iron::prelude::*;
use iron::status;
use iron::mime::*;
use pages;
use db;
use rustache;

pub fn handle(req: &mut Request) -> IronResult<Response> {
    let conn = req.extensions.get::<db::SqlPool>().unwrap().get().unwrap();
    let res = conn.query("SELECT * FROM post ORDER BY created_at DESC", &[]).unwrap();
    let data = pages::data_from_posts(req, res);
    let template = rustache::render_file("tmpl/index.mst", data).unwrap();
    Ok(Response::with((status::Ok,
                       Mime(TopLevel::Text, SubLevel::Html, vec!()),
                       template.unwrap())))
}
