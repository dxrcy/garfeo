use std::{fs, process};

use ibex::prelude::*;
use ibex::{routes, ssg};

mod parse;
use parse::{parse_posts, Post, PostEntry};
mod rss;
use rss::generate_rss;

/// Name of github repo
const URL_ROOT: &str = "/garfeo/";

fn main() {
    let posts = match parse_posts() {
        Ok(posts) => posts,
        Err(err) => {
            eprintln!("\x1b[31;1mFailed to parse posts\x1b[0m");
            eprintln!("\x1b[31m{}\x1b[0m", err);
            process::exit(1)
        }
    };

    let first_last = &FirstLast {
        first: posts.last().expect("no first post"),
        last: posts.first().expect("no last post"),
    };

    let routes = routes![
        (/)
            => at_index(&posts, first_last),
        (/404)
            => at_404(first_last),
        (/"plej-bonaj")
            => at_favourites(&posts, first_last),
        (/"informejo")
            => at_about(first_last),
        (/[entry.post.index])
            for entry in posts.iter()
            => at_post(entry, first_last),
        (/"lasta")
            => at_post(first_last.last, first_last),
    ];

    let rss = generate_rss(&posts, first_last);

    ssg::quick_build(routes).expect("Failed to build");
    fs::write("build/rss.xml", rss).expect("Failed to write RSS file");

    println!("\x1b[34;1mBuilt successfully!\x1b[0m");
}

pub struct FirstLast<'a> {
    first: &'a PostEntry,
    last: &'a PostEntry,
}

fn at_index(entries: &[PostEntry], first_last: &FirstLast) -> Document {
    view! { @use_basic [
        "",
        view!{},
        None,
        first_last,
    ] {
        ol [
            class="post-list",
            reversed=true,
            start=first_last.first.post.index,
        ] {
            [:for PostEntry {post, ..} in entries {
                @list_item [post]
            }]
        }
    }}
    .into()
}

fn at_404(first_last: &FirstLast) -> Document {
    let last_post = &first_last.last.post;
    view! { @use_basic [
        "404",
        view! { "Paĝo ne trovita!" },
        None,
        first_last,
    ] {
        p {
            a [href=url!()] {
                "Reiru al ĉefpaĝo?"
            }
        }

        hr/
        p { b { "Lasta bildstrio:" } }
        ol [start=last_post.index] {
            @list_item [last_post]
        }
    }}
    .into()
}

fn at_favourites(entries: &[PostEntry], first_last: &FirstLast) -> Document {
    view! { @use_basic [
        "",
        view! { "Plej bonaj bildstrioj" },
        None,
        first_last,
    ] {
        ol [class="post-list"] {
            [:for PostEntry {post, ..} in entries {
                [:if post.props.good {
                    @list_item [post]
                }]
            }]
        }
    }}
    .into()
}

fn at_post(entry: &PostEntry, first_last: &FirstLast) -> Document {
    let post = &entry.post;

    view! { @use_basic [
            &format!("{} [{}]", post.title, post.index),
            view!{ @post_title [&post, false] },
            Some(&format!("static/posts/{}/esperanto.png", post.index)),
            first_last,
        ] {
        p [class="details"] {
            span [class="navigate prev"] {
                [:if let Some(prev) = &entry.prev {
                    a [href=url!(&prev.index)] { &laquo }
                }]
            }
            ~
            span [class="text"] {
                "[" span [id="index"] { [&post.index] } "]"
                ~
                a [
                    href=format!("https://gocomics.com/garfield/{}", post.date.replace("-", "/")),
                    title="Spekti je GoComics.com",
                ] {
                    b [id="date"] { [&post.date] }
                }
            }
            ~
            span [class="navigate next"] {
                [:if let Some(next) = &entry.next {
                    a [href=url!(&next.index)] { &raquo }
                }]
            }
        }

        div {
            img [
                id="image-eo",
                class="comic",
                alt="Esperanto bildstrio",
                src=url!(format!("static/posts/{}/esperanto.png", &post.index)),
                height=400,
            ]/
            img [
                id="image-en",
                class="comic",
                alt="Angla bildstrio",
                src=url!(format!("static/posts/{}/english.png", &post.index)),
                height=400,
            ]/
        }

        [:if !post.errata.0.is_empty() { div [class="errata"] {
            h2 { "Eraroj:" }
            ol {
                [:for (old, new) in &post.errata.0 { li {
                    b [class="old"] { [old] }
                    ~ &rarr ~
                    b [class="new"] { [new] }
                } }]
            }
        } }]

        div [class="navigate"] {
            [:if let Some(prev) = &entry.prev {
                div [class="prev"] {
                    a [href=url!(&prev.index)] {
                        strong { "Antaŭa:" } ~
                        @post_title [&prev, true]
                    }
                }
            }]
            [:if let Some(next) = &entry.next {
                div [class="next"] {
                    a [href=url!(&next.index)] {
                        strong { "Sekva:" } ~
                        @post_title [&next, true]
                    }
                }
            }]
        }

        hr/

        div [class="caption"] {
            HEAD { script { [include_str!("js/copy.js")] } }
            pre [id="caption", onclick="copy(this)"] {
                [&post.title] ~ "💚" "&#10;&#10;"
                 "#esperanto #garfield #mondodakomiksoj"
                ~ "[" [&post.index] "]"
            }
        }
    }}
    .into()
}

fn at_about(first_last: &FirstLast) -> Document {
    view! { @use_basic [
        "Informejo",
        view! { "Informejo" },
        None,
        first_last,
    ] {

        h2 { "Kio estas Garfield-EO?" }
        p {
            "Mi tradukas bildstriojn de Garfildo en Esperanton."
            br/
            "Parto de" ~ i { "Mondo da Komiksoj" } "."
        }

        h2 { "Ligiloj" }
        ul [class="links"] {
            li { a [href="https://github.com/darccyy/garfeo"]
                { strong { "Fonta Kodo kaj ĉiu bildstrio" }
                    ~ "- por ĉi tiu retejo (en la angla)" }
            }
            li { a [href="https://github.com/darccyy/garfeo/issues/new"]
                { strong { "Mi havas concernon!" }
                    ~ "- Informu min per GitHub" }
            }
            li { a [href="https://github.com/darccyy/everygarf"]
                { strong { "EveryGarf" }
                    ~ "- Elŝuti ĉiujn Garfildajn bildstriojn ĝisnune" }
            }
            li { a [href="https://mastodon.world/@garfieldeo"]
                { strong { "Mastodon @garfieldeo@mastodon.world" }
                    ~ "- Esperantaj Garfildaj bildstrioj" }
            }
            li { a [href="https://instagram.com/garfield.eo"]
                { strong { "Instagram @garfield.eo" }
                    ~ "- Esperantaj Garfildaj bildstrioj" }
            }
            li { a [href="https://instagram.com/mondodakomiksoj"]
                { strong { "Mondo da Komiksoj" }
                    ~ "- Grupo de tradukistoj" }
            }
            li { a [href=url!("rss.xml")]
                { strong { "RSS-fluo" }
                    ~ "- Aboni la RSS-fluon por novajn bildstriojn" }
            }
        }

        hr/
        br/
        img [
            class="icon-image",
            src=url!("static/icon.png"),
            alt="La vizaĝo de Garfildo",
            height=400,
        ]/
    }}
    .into()
}

fn use_basic(
    title: &str,
    header: View,
    image: Option<&str>,
    first_last: &FirstLast,
    children: View,
) -> View {
    let mut full_title = "Garfildo Esperanta".to_string();
    if !title.is_empty() {
        full_title += " - ";
        full_title += title
    };

    view! {
        HEAD {
            @use_meta [ Meta::new()
                .url(url!())
                .title(&full_title)
                .desc("Legu 500+ bildstrioj de Garfildo, tradukitaj en Esperanton!")
                .image(&url!(image.unwrap_or("static/icon.png")))
                .color("#ffb24e")
                .author("darcy")
                .large_image(true)
            ]

            title { [full_title] }
            link [rel="shortcut icon", href=url!("static/icon.png")]/
            link [rel="stylesheet",    href=url!("css/main.css")]/
        }

        div [class="top-header"] {
            a [class="title", href=url!()] {
                b { "Garfildo Esperanta" }
            }

            div [class="actions"] {
                HEAD { script { [include_str!("js/random.js")] } }
                a [id="random", title="Klaku por iri al iun bildstrio"] {
                    i { "Arbitra" }
                    span [class="icon"] { "⚄" }
                }

                span [class="fallback-divider"] { ~ } // fallback for css
                a [href=url!("informejo")] {
                    i { "Informejo" }
                    span [class="icon smaller"] { "🛈 " }
                }

                span [class="fallback-divider"] { ~ }
                a [href=url!("plej-bonaj")] {
                    i { "Plej Bonaj" }
                    span [class="icon"] { "★" }
                }
            }

            [:use {
                let first = &first_last.first.post.index;
                let last = &first_last.last.post.index;
            } {
                script { [format!("set_random_url('{}', '{}', '{}')", url!(), first, last)] }
            }]
        }
        hr/

        div [class="content-container"] { div [class="content"] {
            h1 [class="header"] {
                [header]
            }
            [children]
        } }
    }
}

fn list_item(post: &Post) -> View {
    view! {
        li [value=post.index] {
            a [href=url!(post.index)] {
                @post_title [post, false]
                img [
                    alt="Antaŭrigardo de Esperanta bildstro",
                    src=url!(format!("static/posts/{}/esperanto.png", post.index)),
                    loading="lazy",
                    height=200, // fallback for css
                ]/
            }
        }
    }
}

fn post_title(post: &Post, italic: bool) -> View {
    let inner = view! {
        // Bold if sunday
        [:if post.sunday {
            b { [ &post.title ] }
        } else {
            [ &post.title ]
        }]
    };

    let inner = view! {
        // Grey if no text
        [:if post.props.notext {
            span [class="gray"] { [inner] }
        } else {
            [inner]
        }]
    };

    view! {
        span [class="title"] {
            // Italic if argument given
            [:if italic {
                i { [inner] }
            } else {
                [inner]
            }]

            // Star if favorite
            [:if post.props.good {
                ~ span [id="good", title="Bona bildstrio"] { "⭐" }
            }]
        }
    }
}
