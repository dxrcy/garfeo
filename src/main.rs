use ibex::prelude::*;

mod blogs;
use blogs::BlogPost;

fn main() {
    let blogs = blogs::get_blog_posts();

    let routes = routes![
        (/)
            => index_page(&blogs),
        (/post/[i])
            for (i, blog) in blogs.into_iter().enumerate()
            => blog_page(blog),
    ];

    let files = route::render_routes(routes);
    route::write_files(files).unwrap();
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
                        ~ "-" ~
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

        image [src=[:?blog.image]]/
    }
    .into()
}

fn header(home_link: bool) -> View {
    let text = "My Website";

    view! {
        HEAD {
            title { [text] }
        }
        h1 {
            [text]
            [*if (home_link) {
                small {
                    ~ "-" ~
                    a [href="/"] { "Back to home page" } }
                }
            ]
        }
    }
}
