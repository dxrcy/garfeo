use ibex::prelude::*;

#[macro_use]
mod features;
mod parse;

use parse::{parse_posts, PostEntry};

const URL_ROOT: &str = "/garfeo-ibex";

fn main() {
    println!("Parsing...");
    let posts = parse_posts();

    println!("Routing...");
    let routes = routes![
        (/)
            => index_page(&posts),
        (/posts/[entry.post.index])
            for entry in posts.into_iter()
            => post_page(entry),
    ];

    println!("Rendering...");
    let files = route::render_routes(routes);
    println!("Writing...");
    route::write_files(files).unwrap();
}

fn index_page(posts: &[PostEntry]) -> Document {
    view! {
        @header[false]

        ul {
            [foreach![PostEntry {post, ..} in posts.into_iter() =>
                li {
                    a [href=[:?url!(format!("/posts/{}", post.index))]] {
                        b { [&post.title] }
                    }
                }
            ]]
        }
    }
    .into()
}

fn post_page(entry: PostEntry) -> Document {
    let PostEntry { post, prev, next } = entry;

    view! {
        @header[true]

        h2 { [post.title] }

        img [src=[:?format!("https://darccyy.github.io/garfield-eo/public/posts/{}/esperanto.png", post.index)]]/
    }
    .into()
}

fn header(home_link: bool) -> View {
    let text = "Garfildo Esperanta";

    view! {
        HEAD {
            title { [text] }
        }
        h1 {
            [text]
            [*if (home_link) {
                small {
                    ~ "-" ~
                    a [href={url!("/")}] { "Back to home page" } }
                }
            ]
        }
    }
}
