macro_rules! assets_url {
    ($path:expr) => {
        if ::ibex::is_local() {
            "/assets/"
        } else {
            "https://raw.githubusercontent.com/dxrcy/garfeo/master/assets/"
        }
        .to_string()
            + &$path
    };
}

use ibex::extras::wrap_if;
use ibex::prelude::*;
use ibex::ssg;

use crate::posts::Post;
use crate::posts::PostRef;
use crate::posts::Special;
use crate::posts::{transcript, PostList};

pub mod icons {
    pub const GOOD: char = '⭐';
    pub const TRANSCRIPT: char = '📜';
    pub const OLD: char = '🟥';
    pub const NOT_OLD: char = '✅';
    pub const REVISED: char = '🔃';
    pub const HALLOWEEN: char = '🎃';
    pub const CHRISTMAS: char = '🎅';
    pub const NEW_YEARS: char = '🎉';
}

pub const SITE_DESCRIPTION: &str = "Legu 900+ bildstrioj de Garfildo, tradukitaj en Esperanton!";

pub fn use_base(title: &str, image: Option<&str>, posts: &PostList, children: View) -> View {
    let mut full_title = "Garfildo Esperanta".to_string();
    if !title.is_empty() {
        full_title = format!("{} - {}", title, full_title);
    };

    view! {
        HEAD {
            @use_meta [ibex::meta! {
                url: url!(),
                title: &full_title,
                desc: SITE_DESCRIPTION,
                image: url!(image.unwrap_or("static/icon.png")),
                author: "darcy",
                color: "#ffb24e",
                large_image: true,
            }]

            title { [full_title] }
            link [rel="shortcut icon", href=url!("static/icon.png")]/
            link [rel="stylesheet",    href=url!("css/base.css")]/
            @ssg::use_autoreload []

            script { [format!("const BASE_URL = '{}'", url!())] }
            script { [format!("const LAST_INDEX = {}",  posts.last().index.as_int())] } // as_int

            script { [include_str!("js/navigate.js")] }
            script { [include_str!("js/random.js")] }
            script { [include_str!("js/goto.js")] }
        }

        @top_header[]

        main ."manual-width" {
            [children]
        }

        footer {
            a [href="https://dxrcy.dev"] {
                "kreita de darcio"
            }
        }
    }
}

fn top_header() -> View {
    view! {
        div ."header" {
            h1 ."title" {
                a [href=url!()] {
                    "Garfildo Esperanta"
                }
            }

            h2 ."actions" {
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

            script { "set_random_url()" }
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
                    src=assets_url!(format!("posts/{}/esperanto.png", post.index)),
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
            [wrap_if(   italic,         |x| view! { i {[x]} },
                wrap_if(post.is_sunday, |x| view! { b {[x]} },
                    view! { [&post.title] }
                )
            )]

            // Star if favorite
            ~ [:if post.props.good {
                span [title="Bona bildstrio"] { [icons::GOOD] }
            }]
            // Icon if special occasion
            ~ @post_special_icon [post.special]
        }
    }
}

// move to `post_title` inline, if `at_list` is removed
pub fn post_special_icon(special: Option<Special>) -> View {
    let Some(special) = special else {
        return view! {};
    };
    let (icon, message) = match special {
        Special::Halloween => (icons::HALLOWEEN, "Feliĉan Halovenon"),
        Special::Christmas => (icons::CHRISTMAS, "Feliĉan Kristnaskon"),
        Special::NewYears => (icons::NEW_YEARS, "Feliĉan Novjaron"),
    };
    view! {
        span [title=message] {
            [icon]
        }
    }
}

pub fn post_transcript(transcript: &transcript::Transcript) -> View {
    view! {
        div ."transcript" {
            h2 { "Transskribo" }
            br/

            [:for (i, panel) in transcript.panels().iter().enumerate() {
                [:if i > 0 { hr ."separator"/ }]

                div ."panel" {
                    [:if panel.lines.is_empty() {
                        div ."empty" { "..." }
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
                                        h4 { [:where let name = sentence_case(name, false) {
                                            [:if *uncommon {
                                                em { [name] }
                                            } else {
                                                [name]
                                            }]
                                        }] }
                                        p ."speech" {
                                            [format_emphasis(&sentence_case(text, false))]
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

pub fn post_copy_caption(post: &Post) -> View {
    view! {
        [&post.title] ~ "💚" "&#10;"
        "#esperanto #garfield #mondodakomiksoj"
        ~ "[" [&post.index()] "]"
    }
}
pub fn post_copy_transcript(post: &Post) -> View {
    let Some(transcript) = &post.transcript else {
        return view! {};
    };
    view! {
        [:for (i, panel) in transcript.panels().iter().enumerate() {
            [:if i > 0 { "\n---" } ]
            [:for transcript::Line { speaker, text } in &panel.lines {
               "\n"
                [match speaker {
                    transcript::Speaker::Sound => view! {
                        "*" [text] "*"
                    },
                    transcript::Speaker::Text => view! {
                        "[" [text] "]"
                    },
                    transcript::Speaker::Character{ name, .. } => view! {
                        @sentence_case[name, false]
                        ":" ~
                        @sentence_case[text, false]
                    }
                }]
            }]
        }]
    }
}

pub fn sentence_case(string: &str, every_word: bool) -> String {
    let mut output = String::new();
    let mut was_punctuation = true;

    for ch in string.chars() {
        output.push(if was_punctuation {
            ch.to_uppercase().next().unwrap_or(ch) // supports esperanto characters
        } else {
            ch
        });
        match ch {
            // End of sentence
            '.' | '!' | '?' => was_punctuation = true,
            // Space, only if every word is capitalized
            ' ' if every_word => was_punctuation = true,
            // Ignore any punctuation
            _ if ch.is_ascii_punctuation() => (),
            // Any other character
            _ => was_punctuation = false,
        }
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
    if is_emphasis {
        output += "</em>";
    }
    output
}
