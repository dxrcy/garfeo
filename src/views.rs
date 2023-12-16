use ibex::extras::wrap_if;
use ibex::prelude::*;
use ibex::ssg;

use crate::posts::PostRef;

use super::posts::{transcript, PostList};
use super::URL_ROOT;

pub const STAR: &str = "⭐";

pub fn use_base(
    title: &str,
    header: View,
    image: Option<&str>,
    posts: &PostList,
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
            @ssg::use_autoreload []
        }

        @top_header [posts]

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

fn top_header(posts: &PostList) -> View {
    view! {
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
                let first = &posts.first().index;
                let last = &posts.last().index;
             {
                script { [format!("set_random_url('{}', '{}', '{}')", url!(), first, last)] }
            }]
        }
        hr/
    }
}

pub fn list_item(post_ref: &PostRef) -> View {
    let post = post_ref.get();
    view! {
        li [value=post.index] {
            a [href=url!(post.index())] {
                @post_title [post_ref, false]
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

pub fn post_title(post: &PostRef, italic: bool) -> View {
    let post = post.get();

    view! {
        span ."title" {
            [wrap_if(       italic,            |x| view! { i           {[x]} },
                wrap_if(    post.props.notext, |x| view! {span ."gray" {[x]} },
                    wrap_if(post.is_sunday,    |x| view! { b           {[x]} },
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

pub fn post_transcript(transcript: &transcript::Transcript) -> View {
    view! {
        div ."transcript" {
            h2 { "Transskribo" }
            [:for (i, panel) in transcript.panels().iter().enumerate() {
                div ."panel" {
                    h3 { "Bildo " [i+1] }
                    [:if panel.lines.is_empty() {
                        div ."empty" { "(nenio)" }
                    } else {
                        div ."texts" {
                            [:for transcript::Line { speaker, text } in &panel.lines {
                                [match speaker {
                                    transcript::Speaker::Sound => view! {
                                        p ."sound" { em { [text] } }
                                    },
                                    transcript::Speaker::Text => view! {
                                        p ."text" { code { [text] } }
                                    },
                                    transcript::Speaker::Character{ name, uncommon } => view! {
                                        h4 { [:where let name = sentence_case(name) {
                                            [:if *uncommon {
                                                em { [name] }
                                            } else {
                                                [name]
                                            }]
                                        }] }
                                        p ."speech" {
                                            [format_emphasis(&sentence_case(text))]
                                        }
                                    }
                                }]
                            }]
                        }
                    }]
                }
            }]
        }
    }
}

fn sentence_case(string: &str) -> String {
    let mut output = String::new();
    let mut was_punctuation = true;

    for ch in string.chars() {
        let is_uppercase = was_punctuation;
        match ch {
            // End of sentence
            '.' | '!' | '?' => was_punctuation = true,
            // Ignore any punctuation
            _ if ch.is_ascii_punctuation() => (),
            // Any other character
            _ => was_punctuation = false,
        }
        output.push(if is_uppercase {
            ch.to_ascii_uppercase()
        } else {
            ch
        });
    }

    output
}

fn format_emphasis(string: &str) -> String {
    let mut output = String::new();
    let mut is_emphasis = false;
    for ch in string.chars() {
        if ch == '*' {
            output += if is_emphasis { "</em>" } else { "<em>" };
            is_emphasis ^= true;
            continue;
        }
        output.push(ch);
    }
    output
}
