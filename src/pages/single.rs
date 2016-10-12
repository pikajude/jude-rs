use iron::prelude::*;
use iron::status;
use iron::mime::*;
use router::Router;
use pages;
use db;
use rustache;

pub fn handle(req: &mut Request) -> IronResult<Response> {
    let conn = req.extensions.get::<db::SqlPool>().unwrap().get().unwrap();
    let slug = req.extensions.get::<Router>().unwrap().find("slug").unwrap().to_string();
    let res = conn.query("SELECT * FROM post WHERE slug = $1", &[&slug]).unwrap();
    if res.is_empty() {
        panic!("No posts")
    } else {
        let data = pages::data_from_posts(req, res);
        let template = rustache::render_file("tmpl/single.mst", data).unwrap();
        Ok(Response::with((status::Ok,
                           Mime(TopLevel::Text, SubLevel::Html, vec!()),
                           template.unwrap())))
    }
}
