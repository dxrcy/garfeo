use ibex::prelude::*;
use ibex::{routes, ssg};

mod parse;
use parse::{parse_posts, Post, PostEntry};

/// Name of github repo
const URL_ROOT: &str = "/garfeo/";

fn main() {
    let posts = parse_posts();

    let first_last = [
        posts.last().expect("no first post"),
        posts.first().expect("no last post"),
    ];

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
            => at_post(first_last[1], first_last),
    ];

    ssg::quick_build(routes).expect("Failed to build");
    println!("All done!");
}

fn at_index(entries: &[PostEntry], first_last: [&PostEntry; 2]) -> Document {
    view! {
        @use_basic ["", None, first_last]

        @comic_preview [first_last]

        ol [reversed=true, start=first_last[0].post.index] {
            [:for PostEntry {post, ..} in entries {
                @list_item [post]
            }]
        }
    }
    .into()
}

fn at_404(first_last: [&PostEntry; 2]) -> Document {
    view! {
        @use_basic ["404", None, first_last]

        h1 { "Paĝo ne trovita!" }
        p {
            a [href=url!()] {
                "Reiru al ĉefpaĝo?"
            }
        }

        hr/
        @comic_preview [first_last]
        @post_title[&first_last[0].post, true]
    }
    .into()
}

fn at_favourites(entries: &[PostEntry], first_last: [&PostEntry; 2]) -> Document {
    view! {
        @use_basic ["", None, first_last]

        h1 { "Plej bonaj bildstrioj" }
        ol {
            [:for PostEntry {post, ..} in entries {
                [:if post.props.good {
                    @list_item [post]
                }]
            }]
        }
    }
    .into()
}

fn at_post(entry: &PostEntry, first_last: [&PostEntry; 2]) -> Document {
    let post = &entry.post;

    view! {
        @use_basic [
            &format!("{} [{}]", post.title, post.index),
            Some(&format!("static/posts/{}/esperanto.png", post.index)),
            first_last,
        ]

        h1 [id="title"] {
            @post_title [&post, false]
        }

        p {
            "[" span [id="index"] { [&post.index] } "]" ~
            a [
                href=format!("https://gocomics.com/garfield/{}", post.date.replace("-", "/")),
                title="Spekti je gocomics.com",
            ] {
                b [id="date"] { [&post.date] }
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

        [:if post.props.revised {
            p { i { "(Retradukita post originala)" } }
        }]

        [:if !post.errata.0.is_empty() { div [class="errata"] {
            h2 { "Eraroj:" }
            ol {
                [:for (old, new) in &post.errata.0 { li {
                    b [class="old"] { [old] }
                    ~ "&rarr;" ~
                    b [class="new"] { [new] }
                } }]
            }
        } }]

        div [class="navigate"] {
            [:if let Some(prev) = &entry.prev {
                div [class="prev"] {
                    a [href=url!(&prev.index)] {
                        b { "Antaŭa:" } ~
                        @post_title [&prev, true]
                    }
                }
            }]
            [:if let Some(next) = &entry.next {
                div [class="next"] {
                    a [href=url!(&next.index)] {
                        b { "Sekva:" } ~
                        @post_title [&next, true]
                    }
                }
            }]
        }

        hr/

        div [class="caption"] {
            HEAD { script { [include_str!("js/copy.js")] } }
            pre [id="caption", onclick="copy(this)"] {
                [&post.title] "💚" "&#10;&#10;"
                 "#esperanto #garfield #mondodakomiksoj"
                ~ "[" [&post.index] "]"
            }
        }
    }
    .into()
}

fn at_about(first_last: [&PostEntry; 2]) -> Document {
    view! {
        @use_basic ["Informejo", None, first_last]
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
                a [href="https://github.com/darccyy/garfeo"] {
                    b { "Fonta Kodo kaj ĉiu bildstrio" }
                    ~ "- por ĉi tiu retejo (en la angla)"
                }
            }
            li {
                a [href="https://github.com/darccyy/garfeo/issues/new"] {
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

fn use_basic(title: &str, image: Option<&str>, first_last: [&PostEntry; 2]) -> View {
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
                HEAD { script { [include_str!("js/random.js")] } }
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

            [:use {
                let first = &first_last[0].post.index;
                let last = &first_last[1].post.index;
            } {
                script { [format!("set_random_url('{}', '{}', '{}')", url!(), first, last)] }
            }]
        }
        hr/
    }
}

fn comic_preview(post: [&PostEntry; 2]) -> View {
    view! {
        div [class="comic-preview"] {
            img [
                class="comic preview",
                alt="Esperanto bildstrio de la plej lasta bildstrio",
                src=url!(format!("static/posts/{}/esperanto.png", &post[0].post.index)),
                height=200,
            ]/
        }
    }
}

fn list_item(post: &Post) -> View {
    view! {
        li [value=post.index] {
            a [href=url!(post.index)] {
                @post_title [post, false]
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
                span [id="good", title="Bona bildstrio"] { "⭐" }
            }]
        }
    }
}
