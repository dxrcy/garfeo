use ssg::prelude::*;

mod blogs;
use blogs::{get_blog_posts, BlogPost};

fn main() {
    let blogs = get_blog_posts();

    let routes = routes![
        (/)
            => index_page(&blogs),
        (/post/[i])
            for (i, blog) in blogs.into_iter().enumerate()
            => blog_page(blog),
    ];

    let files = router::render_routes(routes);
    router::write_files(files).unwrap();
}

fn index_page(blogs: &[BlogPost]) -> Document {
    view! {
        @header[false]

        h2 { "Read blogs posts" }

        ul {
            [blogs.into_iter().enumerate().map(|(i, blog)| view! {
                li {
                    a [href=[:?format!("/post/{i}")]] {
                        b { [&blog.title] }
                        span { " - " }
                        i { [&blog.author] }
                    }
                }
            }).collect::<Vec<View>>()]
        }
    }
    .into()
}

fn blog_page(blog: BlogPost) -> Document {
    view! {
        @header[true]

        h2 { [blog.title] }
        h3 { i {[blog.author]} }

        p {
            [blog.body]
        }

        image[src=[:?blog.image]]/
    }
    .into()
}

fn header(home_link: bool) -> View {
    const TITLE: &str = "My Website";
    view! {
        HEAD { title { [TITLE] } }
        h1 {
            [TITLE]
            [*if (home_link) {
                small {
                    span { " - " }
                    a[href="/"] { "Back to home page" } }
                }
            ]
        }
    }
}
