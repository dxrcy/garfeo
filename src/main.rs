use ibex::prelude::*;

#[macro_use]
mod features;
mod files;
mod parse;

use parse::{parse_posts, Post, PostEntry};
use std::path::Path;

const URL_ROOT: &str = "/garfeo-ibex/";

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
        (/informejo)
            => about_page(),
        (/[entry.post.index])
            for entry in posts.into_iter()
            => post_page(entry),
    ];

    println!("Rendering...");
    let files = route::render_routes(routes);
    println!("Writing...");
    route::write_files(files).unwrap();

    // ONLY COPIES POSTS
    println!("Copying static files...");
    files::copy_folder(Path::new("static"), Path::new("build/static"))
        .expect("Failed to copy static files");
}

fn index_page(entries: &[PostEntry]) -> Document {
    let last_index = unsafe { LAST_INDEX.clone() };
    view! {
        @use_basic ["", None]

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
        @use_basic [
            &format!("{} [{}]", post.title, post.index),
            Some(&format!("static/posts/{}/esperanto.png", post.index)),
        ]

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
                src=[:?url!(format!("static/posts/{}/esperanto.png", &post.index))]
                height=400
            ]/

            [*if (post.english) {
                img [
                    id="image-en"
                    alt="Angla bildstrio"
                    src=[:?url!(format!("static/posts/{}/english.png", &post.index))]
                    height=400
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

        hr/

        div [class="captions"] {
            pre [id="caption-mastodon", onclick="select(this)"] {
                [&post.title] "💚&#10;#esperanto #garfield" [&post.index]
            }
            pre [id="caption-instagram", onclick="select(this)"] {
                [&post.title] "💚&#10;&#10;#garfield #esperanto #eo #memeo #memeoj #mdr #esperantisto #language"
            }
            script { [include_str!("js/select.js")] }
        }
    }
    .into()
}

fn about_page() -> Document {
    view! {
        @use_basic ["Informejo", None]

        h1 { "Informejo" }

        h2 { "Kio estas Garfield-EO?" }
        p {
            "Mi tradukas bildstriojn de Garfildo en Esperanton."
            br/
            "Parto de" ~ i { "Mondo da Komiksoj" } "."
        }

        h2 { "Ligiloj" }
        div [class="links"] {
            p {
                a [href="https://github.com/darccyy/garfield-eo"] {
                    b { "Fonta Kodo kaj ĉiu bildstrio" }
                    "- por ĉi tiu retejo (en la angla)"
                }
            }
            p {
                a [href="https://github.com/darccyy/garfield-eo/issues/new"] {
                    b { "Mi havas concernon!" }
                    "- Informu min per GitHub"
                }
            }
            p {
                a [href="https://github.com/darccyy/everygarf"] {
                    b { "EveryGarf" }
                    "- Elŝuti ĉiujn Garfildajn bildstriojn ĝisnune"
                }
            }
            p {
                a [href="https://mastodon.world/@garfieldeo"] {
                    b { "Mastodon @garfieldeo@mastodon.world" }
                    "- Esperantaj Garfildaj bildstrioj (Mastodon)"
                }
            }
            p {
                a [href="https://instagram.com/garfield.eo"] {
                    b { "Instagram @garfield.eo" }
                    "- Esperantaj Garfildaj bildstrioj (Instagram)"
                }
            }
        }

        hr/
        br/

        div {
            img [
                src=[:?url!("static/icon.png")]
                alt="La vizaĝo de Garfildo"
                height=400
            ]/
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

fn use_basic(title: &str, image: Option<&str>) -> View {
    let first_index = unsafe { FIRST_INDEX.clone() };
    let last_index = unsafe { LAST_INDEX.clone() };

    let mut full_title = "Garfildo Esperanta".to_string();
    if !title.is_empty() {
        full_title += " - ";
        full_title += title
    };

    view! {
        HEAD {
            @use_meta [Meta {
                url:    Some(url!()),
                title:  Some(&full_title),
                desc:   Some("Legu 500+ bildstrioj de Garfildo, tradukitaj en Esperanton!"),
                image:  Some(&url!(image.unwrap_or("static/icon.png"))),
                color:  Some("#ffb24e"),
                author: Some("darcy"),
            }]

            title { [full_title] }

            link [rel=[:?"shortcut icon"], href=[:?url!("static/icon.png")]]/

            script { [include_str!("js/random.js")] }
        }

        p [class="header"] {
            a [href=[:?url!()]] {
                "Garfildo Esperanta"
            }

            br/
            span [class="subheader"] {
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

        hr/
    }
}

struct Meta<'a> {
    url: Option<&'a str>,
    title: Option<&'a str>,
    desc: Option<&'a str>,
    image: Option<&'a str>,
    author: Option<&'a str>,
    color: Option<&'a str>,
}

fn use_meta(meta: Meta) -> View {
    view! {
        HEAD {
            meta [charset="utf-8"]/
            meta [name="viewport", content="width=device-width, initial-scale=1"]/

            [if let Some(url) = meta.url { view!{
                meta [name="url",        content=[:?url]]/
                meta [property="og:url", content=[:?url]]/
            }} else { view! {}}]

            [if let Some(title) = meta.title { view!{
                meta [itemprop="name",     content=[:?title]]/
                meta [property="og:title", content=[:?title]]/
                meta [name="title",        content=[:?title]]/
            }} else { view! {}}]

            [if let Some(desc) = meta.desc{ view!{
                meta [name="description",         content=[:?desc]]/
                meta [itemprop="description",     content=[:?desc]]/
                meta [property="og:description",  content=[:?desc]]/
                meta [name="twitter:description", content=[:?desc]]/
            }} else { view! {}}]

            [if let Some(image) = meta.image { view!{
                meta [name="image",         content=[:?image]]/
                meta [itemprop="image",     content=[:?image]]/
                meta [property="og:image",  content=[:?image]]/
                meta [name="twitter:image", content=[:?image]]/
            }} else { view! {}}]

            [if let Some(author) = meta.author { view!{
                meta [name="author", content=[:?author]]/
            }} else { view! {}}]

            [if let Some(color) = meta.color { view!{
                meta [name="theme-color", content=[:?color]]/
            }} else { view! {}}]
        }
    }
}
