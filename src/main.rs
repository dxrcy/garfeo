use ibex::prelude::*;
use ibex::{routes, ssg};

mod parse;
use parse::{parse_posts, Post, PostEntry};

/// Name of github repo
const URL_ROOT: &str = "/garfeo/";

// Global variables
static mut FIRST_INDEX: String = String::new();
static mut LAST_INDEX: String = String::new();

fn main() {
    let posts = parse_posts();

    let last_post_entry = posts.first().expect("no last post");
    unsafe {
        FIRST_INDEX = posts.last().expect("no first post").post.index.clone();
        LAST_INDEX = last_post_entry.post.index.clone();
    }

    let routes = routes![
        (/)
            => at_index(&posts),
        (/404)
            => at_404(),
        (/"plej-bonaj")
            => at_favourites(&posts),
        (/"informejo")
            => at_about(),
        (/[entry.post.index])
            for entry in posts.iter()
            => at_post(entry),
        (/"lasta")
            => at_post(&last_post_entry),
    ];

    ssg::quick_build(routes).expect("Failed to build");
    println!("All done!");
}

fn at_404() -> Document {
    view! {
        @use_basic ["404", None]

        h1 { "Paĝo ne trovita!" }
        p {
            a [href=url!()] {
                "Reiru al ĉefpaĝo?"
            }
        }
    }
    .into()
}

fn at_index(entries: &[PostEntry]) -> Document {
    let last_index = unsafe { LAST_INDEX.clone() };
    view! {
        @use_basic ["", None]

        ol [reversed=true, start=last_index] {
            [*for (PostEntry {post, ..}) in (entries.into_iter()) {
                @list_item [post]
            }]
        }
    }
    .into()
}

fn at_favourites(entries: &[PostEntry]) -> Document {
    view! {
        @use_basic ["", None]

        h1 { "Plej bonaj bildstrioj" }
        ol {
            [*for (PostEntry {post, ..}) in (entries.into_iter()) {
                [*if (post.props.good) {
                    @list_item [post]
                }]
            }]
        }
    }
    .into()
}

fn at_post(entry: &PostEntry) -> Document {
    let post = &entry.post;

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
                href=format!("https://gocomics.com/garfield/{}", slash_date(&post.date)),
                title="Spekti je gocomics.com",
            ] {
                b [id="date"] { [&post.date] }
            }
        }

        div [class="images"] {
            img [
                id="image-eo",
                alt="Esperanto bildstrio",
                src=url!(format!("static/posts/{}/esperanto.png", &post.index)),
                height=400,
            ]/

            img [
                id="image-en",
                alt="Angla bildstrio",
                src=url!(format!("static/posts/{}/english.png", &post.index)),
                height=400,
            ]/
        }

        [*if (post.props.revised) {
            p { i { "(Retradukita post originala)" } }
        }]

        [*if (!post.errata.0.is_empty()) { div [class="errata"] {
            h2 { "Eraroj:" }
            ol {
                [*for ((old, new)) in (post.errata.0.iter()) { li {
                    b [class="old"] { [old] }
                    "->"
                    b [class="new"] { [new] }
                } }]
            }
        } }]

        div [class="navigate"] {
            [if let Some(prev) = &entry.prev { view! {
                div [class="prev"] {
                    a [href=url!(&prev.index)] {
                        b { "Antaŭa:" } ~
                        @title [&prev, true]
                    }
                }
            }} else { view!{} }]
            [if let Some(next) = &entry.next { view! {
                div [class="next"] {
                    a [href=url!(&next.index)] {
                        b { "Sekva:" } ~
                        @title [&next, true]
                    }
                }
            }} else { view!{} }]
        }

        hr/

        div [class="captions"] {
            HEAD {
                script { [include_str!("js/copy.js")] }
            }
            pre [id="caption-mastodon", onclick="copy(this)"] {
                [&post.title] "💚" "&#10;&#10;"
                 "#esperanto #garfield #mondodakomiksoj"
                ~ "[" [&post.index] "]"
            }
            pre [id="caption-instagram", onclick="copy(this)"] {
                [&post.title] "💚" "&#10;&#10;"
                "#esperanto #garfield #mondodakomiksoj #memeo #memeoj #bildstrio #garf #esperantomemes"
                ~ "[" [&post.index] "]"
            }
        }
    }
    .into()
}

fn at_about() -> Document {
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
        ul [class="links"] {
            li {
                a [href="https://github.com/darccyy/garfield-eo"] {
                    b { "Fonta Kodo kaj ĉiu bildstrio" }
                    ~ "- por ĉi tiu retejo (en la angla)"
                }
            }
            li {
                a [href="https://github.com/darccyy/garfield-eo/issues/new"] {
                    b { "Mi havas concernon!" }
                    ~ "- Informu min per GitHub"
                }
            }
            li {
                a [href="https://github.com/darccyy/everygarf"] {
                    b { "EveryGarf" }
                    ~ "- Elŝuti ĉiujn Garfildajn bildstriojn ĝisnune"
                }
            }
            li {
                a [href="https://mastodon.world/@garfieldeo"] {
                    b { "Mastodon @garfieldeo@mastodon.world" }
                    ~ "- Esperantaj Garfildaj bildstrioj"
                }
            }
            li {
                a [href="https://instagram.com/garfield.eo"] {
                    b { "Instagram @garfield.eo" }
                    ~ "- Esperantaj Garfildaj bildstrioj"
                }
            }
            li {
                a [href="https://instagram.com/mondodakomiksoj"] {
                    b { "Mondo da Komiksoj" }
                    ~ "- Grupo de tradukistoj"
                }
            }
        }

        hr/
        br/

        div {
            img [
                src=url!("static/icon.png"),
                alt="La vizaĝo de Garfildo",
                height=400,
            ]/
        }
    }
    .into()
}

fn list_item(post: &Post) -> View {
    view! {
        li [value=post.index] {
            a [href=url!(post.index)] {
                @title [post, false]
            }
            div {
                img [
                    alt="Antaŭrigardo de Esperanta bildstro",
                    src=url!(format!("static/posts/{}/esperanto.png", post.index)),
                    loading="lazy",
                ]/
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

        div [class="header"] {
            a [href=url!()] {
                b { "Garfildo Esperanta" }
            }

            div [class="subheader"] {
                HEAD {
                    script { [include_str!("js/random.js")] }
                }
                a [id="random", title="Klaku por iri al iun bildstrio"] {
                    i { "Arbitra" }
                }
                span [class="divider"] { "|" }
                a [href=url!("informejo")] {
                    i { "Informejo" }
                }
                span [class="divider"] { "|" }
                a [href=url!("plej-bonaj")] {
                    i { "Plej Bonaj" }
                }
            }

            script { [format!("set_random_url({:?}, {}, {})", url!(), first_index, last_index)] }
        }

        hr/
    }
}
