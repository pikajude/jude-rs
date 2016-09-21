use postgres::rows::{Row,Rows};
use rustache::HashBuilder;
use iron;
use chrono;
use highlighting;

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

pub fn data_from_posts<'a,'b>(req: &mut iron::Request, res: Rows<'b>) -> HashBuilder<'a> {
    let ref posts = res.iter().map(from_row).collect::<Vec<_>>();
    HashBuilder::new().insert_vector("posts", |builder| {
        let mut builder_tmp = builder;
        for ref post in posts {
            builder_tmp = builder_tmp.push_hash(|hbuilder| {
                hbuilder.insert_string("title", post.title.to_string())
                    .insert_string("link", url_for!(req, "single", "slug" => post.slug.to_string()))
                    .insert_string("content", highlighting::highlighted_markdown(post.content.clone()))
                    .insert_string("created_at", format!("{:?}", post.created_at))
            })
        }
        builder_tmp
    })
    .set_partials_path("tmpl")
}

pub mod home;
pub mod single;
