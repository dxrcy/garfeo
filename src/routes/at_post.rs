use ibex::prelude::*;

use crate::posts::{PostRef, Special};
use crate::views::{
    post_copy_caption, post_copy_transcript, post_title, post_transcript, use_base,
    GITHUB_FILES_URL,
};

pub fn at_post(post_ref: PostRef) -> Document {
    let post = post_ref.get();

    document! { [lang="eo"] @use_base [
        &format!("{} [{}]", post.title, post.index),
        Some(&assets_url!(format!("posts/{}/esperanto.png", post.index))),
        post_ref.list(),
    ] {
        HEAD {
            script { [include_str!("../js/copy.js")] }
            script { [format!("register_copy_key('{}')", post.index().to_string())] }
        }

        h2 { @post_title[&post_ref, false] }

        p ."details" {
            span ."navigate prev" {
                [:if let Some(prev) = &post_ref.prev() {
                    HEAD { script { [format!("register_navigation_key('{}', '{}')", "p", prev.index())] }}
                    a [href=url!(&prev.index())] { &laquo }
                }]
            }
            ~
            span ."text" {
                "[" span ."index" [onclick="copy_text(this.innerText)"] { [&post.index()] } "]"
                ~
                a [
                    href=format!("https://gocomics.com/garfield/{}", post.date.replace('-', "/")),
                    title="Spekti je GoComics.com",
                ] {
                    b { [&post.date] }
                }
            }
            ~
            span ."navigate next" {
                [:if let Some(next) = &post_ref.next() {
                    HEAD { script { [format!("register_navigation_key('{}', '{}')", "n", next.index())] }}
                    a [href=url!(&next.index())] { &raquo }
                }]
            }
        }

        div {
            img ."comic" [
                alt="Esperanta bildstrio",
                src=assets_url!(format!("posts/{}/esperanto.png", &post.index)),
                height=400,
            ]/
            img ."comic" [
                alt="Angla bildstrio",
                src=assets_url!(format!("posts/{}/english.png", &post.index)),
                height=400,
            ]/
        }

        div ."small gray" {
            [:if post.is_old {
                "(olda)"
            }]
        }

        div ."navigate" {
            [:if let Some(prev) = &post_ref.prev_ref() {
                div ."prev" {
                    a [href=url!(&prev.get().index())] {
                        strong { "Antaŭa:" } ~
                        @post_title [prev, true]
                    }
                }
            }]
            [:if let Some(next) = &post_ref.next_ref() {
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
                Some(Special::Halloween) => view! { "Feliĉan Halovenon!" },
                Some(Special::Christmas) => view! { "Feliĉan Kristnaskon!" },
                Some(Special::NewYears) => view! { "Feliĉan Novjaron!" },
                _ => view! {},
            }]
        }

        [:if let Some(transcript) = &post.transcript {
            @post_transcript [transcript]
        }]

        hr/

        div ."copyable" {
            div { pre [onclick="copy_text(this.innerText)"] { @post_copy_caption[&post] } }
            div { pre [onclick="copy_text(this.innerText)"] { @post_copy_transcript[&post] } }
        }

        a ."source" [
            href=format!("{}/static/posts/{}", GITHUB_FILES_URL, post.index),
        ] {
            "Vidu fonton"
        }

        [:if post.is_revised {
            div ."old-version" {
                details {
                    summary {
                        "Vidu malnovan version"
                    }
                    img ."comic" [
                        alt="Malnova esperanta bildstrio",
                        src=assets_url!(format!("old/{}/esperanto.png", &post.index)),
                        height=400,
                    ]/
                }
            }
        }]
    }}
}
