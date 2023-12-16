use ibex::prelude::*;

use crate::posts::{PostRef, Special};

use crate::views::{post_title, post_transcript, use_base};
use crate::URL_ROOT;

pub fn at_post(post_ref: PostRef) -> Document {
    let post = post_ref.get();

    view! { @use_base [
        &format!("{} [{}]", post.title, post.index),
        post_title(&post_ref, false),
        Some(&format!("static/posts/{}/esperanto.png", post.index)),
        post_ref.list(),
    ] {
        p ."details" {
            span ."navigate prev" {
                [:if let Some(prev) = &post_ref.prev() {
                    a [href=url!(&prev.index())] { &laquo }
                }]
            }
            ~
            span ."text" {
                "[" span #"index" { [&post.index()] } "]"
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
                [:if let Some(next) = &post_ref.next() {
                    a [href=url!(&next.index())] { &raquo }
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

        div ."small gray" {
            [:if post.version > 0 {
                b { "Revizio:" }
                ~ [post.version]
            }]
            ~
            [:if post.is_old {
                "(olda)"
            }]
        }

        [:if !post.errata.items.is_empty() { div ."errata" {
            h2 { "Eraroj:" }
            ol {
                [:for (old, new) in &post.errata.items { li {
                    b ."old" { [old] }
                    ~ &rarr ~
                    b ."new" { [new] }
                } }]
            }
        } }]

        div ."navigate" {
            [:if let Some(prev) = &post_ref.next_ref() {
                div ."prev" {
                    a [href=url!(&prev.get().index())] {
                        strong { "Antaŭa:" } ~
                        @post_title [prev, true]
                    }
                }
            }]
            [:if let Some(next) = &post_ref.prev_ref() {
                div ."next" {
                    a [href=url!(&next.get().index())] {
                        strong { "Sekva:" } ~
                        @post_title [&next, true]
                    }
                }
            }]
        }

        div ."special" {
            [match &post.special {
                Some(Special::Christmas) => view! { "Feliĉan Kristnaskon!" },
                Some(Special::Halloween) => view! { "Feliĉan Halovenon!" },
                _ => view! {},
            }]
        }

        [:if let Some(transcript) = &post.transcript {
            @post_transcript [transcript]
        }]

        hr/

        div ."caption" {
            HEAD { script { [include_str!("../js/copy.js")] } }
            pre #"caption" [onclick="copy(this)"] {
                [&post.title] ~ "💚" "&#10;&#10;"
                 "#esperanto #garfield #mondodakomiksoj"
                ~ "[" [&post.index()] "]"
            }
        }

        a ."source" [
            href=format!("https://github.com/darccyy/garfeo/tree/master/static/posts/{}", post.index),
        ] {
            "Vidu fonton"
        }
    }}
    .into()
}
