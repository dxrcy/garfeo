use std::process;

use ibex::extras::wrap_if;
use ibex::prelude::*;
use ibex::{routes, ssg};

mod parse;
use parse::{parse_posts, Post, PostEntry};
mod rss;

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
        // Normal pages
        (/)
            => at_index(&posts, first_last),
        (/404)
            => at_404(first_last),
        (/"plej-bonaj")
            => at_favourites(&posts, first_last),
        (/"informejo")
            => at_about(first_last),
        (/"listo")
            => at_list(&posts, first_last),

        // Posts (HTML)
        (/[entry.post.index])
            for entry in posts.iter()
            => at_post(entry, first_last),
        (/"lasta")
            => at_post(first_last.last, first_last),

        // Posts (JSON)
        (/"index.json") | (/"posts.json")
            => ssg::raw(json_index(&posts)),
        (/[entry.post.index]".json")
            for entry in posts.iter()
            => ssg::raw(entry.to_json()),
        (/"latest.json")
            => ssg::raw(first_last.last.to_json()),

        // RSS file
        (/"rss.xml")
            => ssg::raw(rss::generate_rss(&posts, first_last)),
    ];

    ssg::quick_build(routes).expect("Failed to build");
    println!("\x1b[34;1mBuilt successfully!\x1b[0m");
}

pub struct FirstLast<'a> {
    first: &'a PostEntry,
    last: &'a PostEntry,
}

fn at_index(entries: &[PostEntry], first_last: &FirstLast) -> Document {
    view! { @use_base [
        "",
        view!{},
        None,
        first_last,
    ] {
        ol ."post-list" [
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
    view! { @use_base [
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
    view! { @use_base [
        "Plej bonaj",
        view! { "Plej bonaj bildstrioj" },
        None,
        first_last,
    ] {
        ol ."post-list" {
            [:for PostEntry {post, ..} in entries {
                [:if post.props.good {
                    @list_item [post]
                }]
            }]
        }
    }}
    .into()
}

fn at_list(entries: &[PostEntry], first_last: &FirstLast) -> Document {
    view! { @use_base [
        "Alia listo",
        view!{},
        None,
        first_last,
    ] {
        table ."graph" {
            [:for PostEntry { post, .. } in entries {
                tr {
                    td { [:if post.props.good { [STAR] }] }
                    td { a [href=url!(post.index), title=post.title] {
                        [:if post.sunday
                            { b  { [&post.index] } }
                            else { [&post.index] }
                        ]
                    }}
                    td { [:for _ in 0..post.version { span { "🟥" } }] }
                }
            }]
        }

    }}
    .into()
}

fn at_post(entry: &PostEntry, first_last: &FirstLast) -> Document {
    let post = &entry.post;

    view! { @use_base [
            &format!("{} [{}]", post.title, post.index),
            view!{ @post_title [&post, false] },
            Some(&format!("static/posts/{}/esperanto.png", post.index)),
            first_last,
        ] {
        p ."details" {
            span ."navigate prev" {
                [:if let Some(prev) = &entry.prev {
                    a [href=url!(&prev.index)] { &laquo }
                }]
            }
            ~
            span ."text" {
                "[" span #"index" { [&post.index] } "]"
                ~
                a [
                    href=format!("https://gocomics.com/garfield/{}", post.date.replace('-', "/")),
                    title="Spekti je GoComics.com",
                ] {
                    b #"date" { [&post.date] }
                }
            }
            ~
            span ."navigate next" {
                [:if let Some(next) = &entry.next {
                    a [href=url!(&next.index)] { &raquo }
                }]
            }
        }

        div {
            img #"image-eo" ."comic" [
                alt="Esperanto bildstrio",
                src=url!(format!("static/posts/{}/esperanto.png", &post.index)),
                height=400,
            ]/
            img #"image-en" ."comic" [
                alt="Angla bildstrio",
                src=url!(format!("static/posts/{}/english.png", &post.index)),
                height=400,
            ]/
        }

        [:if post.version > 0 {
            p ."small gray" {
                b { "Revizio:" }
                ~ [post.version]
            }
        }]

        [:if !post.errata.0.is_empty() { div ."errata" {
            h2 { "Eraroj:" }
            ol {
                [:for (old, new) in &post.errata.0 { li {
                    b ."old" { [old] }
                    ~ &rarr ~
                    b ."new" { [new] }
                } }]
            }
        } }]

        div ."navigate" {
            [:if let Some(prev) = &entry.prev {
                div ."prev" {
                    a [href=url!(&prev.index)] {
                        strong { "Antaŭa:" } ~
                        @post_title [&prev, true]
                    }
                }
            }]
            [:if let Some(next) = &entry.next {
                div ."next" {
                    a [href=url!(&next.index)] {
                        strong { "Sekva:" } ~
                        @post_title [&next, true]
                    }
                }
            }]
        }

        hr/

        div ."caption" {
            HEAD { script { [include_str!("js/copy.js")] } }
            pre #"caption" [onclick="copy(this)"] {
                [&post.title] ~ "💚" "&#10;&#10;"
                 "#esperanto #garfield #mondodakomiksoj"
                ~ "[" [&post.index] "]"
            }
        }
    }}
    .into()
}

fn at_about(first_last: &FirstLast) -> Document {
    view! { @use_base [
        "Informejo",
        view! { "Informejo" },
        None,
        first_last,
    ] {

        h3 { "Kio estas Garfield-EO?" }
        p {
            "Mi tradukas bildstriojn de Garfildo en Esperanton."
            br/
            "Parto de" ~ i { "Mondo da Komiksoj" } "."
        }

        h3 { "Ligiloj" }
        ul ."links" {
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
        img ."icon-image" [
            src=url!("static/icon.png"),
            alt="La vizaĝo de Garfildo",
            height=400,
        ]/
    }}
    .into()
}

fn use_base(
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
            @use_meta [ibex::meta! {
                url: url!(),
                title: &full_title,
                desc: "Legu 500+ bildstrioj de Garfildo, tradukitaj en Esperanton!",
                image: url!(image.unwrap_or("static/icon.png")),
                author: "darcy",
                color: "#ffb24e",
                large_image: true,
            }]

            title { [full_title] }
            link [rel="shortcut icon", href=url!("static/icon.png")]/
            link [rel="stylesheet",    href=url!("css/base.css")]/
            @ssg::use_autoreload
        }

        div ."header" {
            h1 ."title" {
                a [href=url!()] {
                    "Garfildo Esperanta"
                }
            }

            h2 ."actions" {
                HEAD { script { [include_str!("js/random.js")] } }
                a #"random" [title="Klaku por iri al iun bildstrio"] {
                    i { "Arbitra" }
                    span ."icon" { "⚄" }
                }

                span ."fallback-divider" { ~ } // fallback for css
                a [href=url!("informejo")] {
                    i { "Informejo" }
                    span ."icon smaller" { "🛈 " }
                }

                span ."fallback-divider" { ~ }
                a [href=url!("plej-bonaj")] {
                    i { "Plej Bonaj" }
                    span ."icon" { "★" }
                }
            }

            [:where
                let first = &first_last.first.post.index;
                let last = &first_last.last.post.index;
             {
                script { [format!("set_random_url('{}', '{}', '{}')", url!(), first, last)] }
            }]
        }
        hr/

        article ."manual-width" {
            [:if !header.is_empty() {
                h2 { [header] }
            }]
            [children]
        }

        footer {
            a [href="https://darccyy.github.io"] {
                "kreita de darcio"
            }
        }
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

const STAR: &str = "⭐";

fn post_title(post: &Post, italic: bool) -> View {
    view! {
        span ."title" {
            [wrap_if(       italic,            |x| view! { i           {[x]} },
                wrap_if(    post.props.notext, |x| view! {span ."gray" {[x]} },
                    wrap_if(post.sunday,       |x| view! { b           {[x]} },
                        view! { [&post.title] }
                    )
               )
            )]

            // Star if favorite
            [:if post.props.good {
                ~ span #"good" [title="Bona bildstrio"] { [STAR] }
            }]
        }
    }
}

fn json_index(entries: &[PostEntry]) -> String {
    format!(
        "[\n    {}\n]",
        entries
            .into_iter()
            .map(|entry| entry.post.to_json())
            .collect::<Vec<_>>()
            .join(",\n")
            .replace('\n', "\n    "),
    )
}
