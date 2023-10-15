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

macro_rules! foreach {
    ( $pat:pat in $expr:expr => $($tt:tt)* ) => {
        ($expr).map(|$pat| view! { $($tt)* }).collect::<Vec<View>>()
    }
}

fn index_page(blogs: &[BlogPost]) -> Document {
    view! {
        @header[false]

        h2 { "Read blogs posts" }
        ul {
            // until `for` works in macro
            [foreach![(i, blog) in blogs.into_iter().enumerate() =>
                li {
                    a [href=[:?url(format!("/post/{i}"))]] {
                        b { [&blog.title] }
                        ~ "-" ~
                        i { [&blog.author] }
                    }
                }
            ]]
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

        img [src=[:?url(blog.image)]]/
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
                    a [href={url("/")}] { "Back to home page" } }
                }
            ]
        }
    }
}

fn url(link: impl Into<String>) -> String {
    let root = if std::env::args().nth(1) == Some("local".to_string()) {
        ""
    } else {
        "/ibex-example"
    };
    format!("{}{}", root, link.into())
}
