create table if not exists post (
  id serial primary key,
  title text not null,
  slug text not null unique,
  content text not null,
  created_at timestamp with time zone not null
);

insert into post (title, slug, content, created_at)
values ('My blog post', 'my-blog-post', E'A new blog post\r\n\r\n    This is my new blog post\r\n\r\n', now());
