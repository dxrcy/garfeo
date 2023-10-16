use ibex::prelude::*;

#[macro_use]
mod features;
mod parse;

use parse::{parse_posts, Post, PostEntry};

const URL_ROOT: &str = "/garfeo-ibex";

static mut FIRST_INDEX: String = String::new();
static mut LAST_INDEX: String = String::new();

fn main() {
    println!("Parsing...");
    let posts = parse_posts();

    unsafe {
        FIRST_INDEX = posts.last().expect("no last post").post.index.clone();
        LAST_INDEX = posts.first().expect("no first post").post.index.clone();
    }

    println!("Routing...");
    let routes = routes![
        (/)
            => index_page(&posts),
        (/[entry.post.index])
            for entry in posts.into_iter()
            => post_page(entry),
    ];

    println!("Rendering...");
    let files = route::render_routes(routes);
    println!("Writing...");
    route::write_files(files).unwrap();
}

fn index_page(entries: &[PostEntry]) -> Document {
    let last_index = unsafe { LAST_INDEX.clone() };
    view! {
        @header

        ol [reversed, start=last_index] {
            [*for (PostEntry {post, ..}) in (entries.into_iter()) {
                @list_item [post]
            }]
        }
    }
    .into()
}

fn post_page(entry: PostEntry) -> Document {
    let post = entry.post;

    view! {
        @header

        h1 [id="title"] {
            @title [&post, false]
        }

        p {
            "[" span [id="index"] { [&post.index] } "]" ~
            a [
                href=[:?format!("https://gocomics.com/garfield/{}", slash_date(&post.date))]
                title="Spekti je gocomics.com"
            ] {
                b [id="date"] { [&post.date] }
            }
        }

        div [class="images"] {
            img [
                id="image-eo"
                alt="Esperanto bildstrio"
                src=[:?format!("https://darccyy.github.io/garfield-eo/public/posts/{}/esperanto.png", &post.index)]
            ]/

            [*if (post.english) {
                img [
                    id="image-en"
                    alt="Angla bildstrio"
                    src=[:?format!("https://darccyy.github.io/garfield-eo/public/posts/{}/esperanto.png", &post.index)]
                ]/
            } else {
                br/
                span [id="no-image-en"] { "(Mankas angla bildstrio)" }
            }]
        }

        [*if (post.props.revised) {
            p { i { "(Retradukita post originala)" } }
        }]

        [*if (!post.errata.0.is_empty()) { div [class="errata"] {
            h2 { "Eraroj:" } 
            ol {
                [*for ((old, new)) in (post.errata.0.into_iter()) { li {
                    b [class="old"] { [old] }
                    "->"
                    b [class="new"] { [new] }
                } }]
            }
        } }]

        div [class="navigate"] {
            [if let Some(prev) = entry.prev { view! {
                a [
                    class="prev"
                    href=[:?url!(&prev.index)]
                ] {
                    b { "Antaŭa:" } ~
                    @title [&prev, true]
                }
            }} else { view!{} }]

            [if let Some(next) = entry.next { view! {
                a [
                    class="next"
                    href=[:?url!(&next.index)]
                ] {
                    b { "Sekva:" } ~
                    @title [&next, true]
                }
            }} else { view!{} }]
        }
    }
    .into()
}

fn list_item(post: &Post) -> View {
    view! {
        li {
            a [href=[:?url!(post.index)]] {
                @title [post, false]
            }
        }
    }
}

fn title(post: &Post, italic: bool) -> View {
    let inner = view! {
        // Bold if sunday
        [*if (post.sunday) {
            b { [ &post.title ] }
        } else {
            [ &post.title ]
        }]
    };

    let inner = view! {
        // Grey if no text
        [*if (post.props.notext) {
            span [class="gray"] { [inner] }
        } else {
            [inner]
        }]
    };

    view! {
        span [class="title"] {
            // Italic if argument given
            [*if (italic) {
                i { [inner] }
            } else {
                [inner]
            }]

            // Star if favorite
            [*if (post.props.good) {
                span [id="good", title="Bona bildstrio"] { "⭐" }
            }]
        }
    }
}

fn slash_date(date: &str) -> String {
    date.replace("-", "/")
}

fn header() -> View {
    let first_index = unsafe { FIRST_INDEX.clone() };
    let last_index = unsafe { LAST_INDEX.clone() };

    view! {
        HEAD {
            meta [charset="utf-8"]/
            meta [name="viewport", content="width=device-width, initial-scale=1"]/
            title { "NO TITLE" }
            script { [include_str!("js/random.js")] }
        }

        p [class="header"] {
            a [href=[:?url!()]] {
                "Garfildo Esperanta"
            }
            br/
            span [class="subheader"] {
                "<!-- Requires Javascript :( -->"
                a [id="random", title="Klaku por iri al iun bildstrio"] {
                    i { "Arbitra" }
                }

                span [class="divider"] { "|" }
                a [href=[:?url!("informejo")]] {
                    i { "Informejo" }
                }
                span [class="divider"] { "|" }
                a [href=[:?url!("plej-bona")]] {
                    i { "Plej Bonaj" }
                }
            }

            script { [format!("set_random_url({:?}, {}, {})", url!(), first_index, last_index)] }
        }
    }
}
