use postgres::rows::{Row,Rows};
use rustache::HashBuilder;
use iron;
use chrono;

#[derive(Debug)]
struct Post {
    title: String,
    slug: String,
    content: String,
    created_at: chrono::DateTime<chrono::UTC>
}

fn from_row<'a>(r: Row<'a>) -> Post {
    Post {
        title: r.get("title"),
        slug: r.get("slug"),
        content: r.get("content"),
        created_at: r.get("created_at")
    }
}

pub fn data_from_posts<'a,'b>(req: &iron::Request, res: Rows<'b>) -> HashBuilder<'a> {
    use highlighting;
    use Auth;

    let hb = HashBuilder::new().insert_vector("posts", |mut builder| {
        for ref post in res.iter().map(from_row) {
            builder = builder.push_hash(|hbuilder| {
                hbuilder.insert_string("title", post.title.to_string())
                    .insert_string("link", url_for!(req, "single", "slug" => post.slug.to_string()))
                    .insert_string("content", highlighting::highlighted_markdown(post.content.clone()))
                    .insert_string("created_at", format!("{:?}", post.created_at))
            })
        }
        builder
    })
    .set_partials_path("tmpl");

    if let Some(uname) = req.extensions.get::<Auth>() {
        hb.insert_string("user", uname)
    } else {
        hb
    }
}

pub mod home;
pub mod single;
