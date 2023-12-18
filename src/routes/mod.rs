mod at_post;
mod json;

use ibex::prelude::*;

use crate::posts::{Post, PostList};
use crate::views::{icons, list_item, use_base};
use crate::URL_ROOT;

pub use at_post::*;
pub use json::*;

pub fn at_index(posts: &PostList) -> Document {
    view! { @use_base [
        "",
        view! {},
        None,
        posts,
    ] {
        ol ."post-list" [
            reversed=true,
            start=posts.first().index,
        ] {
            [:for post in posts.into_iter().rev() {
                @list_item [&post]
            }]
        }
    }}
    .into()
}

pub fn at_favorites(posts: &PostList) -> Document {
    view! { @use_base [
        "Plej bonaj",
        view! { "Plej bonaj bildstrioj" },
        None,
        posts,
    ] {
        h3 { "(Laŭ mia opinio)" }
        ol ."post-list" [
            reversed=true,
            start=posts.first().index,
        ] {
            [:for post in posts.into_iter().rev() {
                [:if post.get().props.good {
                    @list_item [&post]
                }]
            }]
        }
    }}
    .into()
}

pub fn at_about(posts: &PostList) -> Document {
    view! { @use_base [
        "Informejo",
        view! { "Informejo" },
        None,
        posts,
    ] {

        h3 { "Kio estas Garfield-EO?" }
        p {
            "Mi tradukas bildstriojn de Garfildo en Esperanton."
            br/
            "Parto de" ~ i{"Mondo da Komiksoj"} "."
        }

        p {
            "Vidu kiel mi kreas kaj alŝutas tradukojn,"
            ~ a [href=url!("instrukcio")] { i{"ĉi tie"} } "."
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

pub fn at_instructions(posts: &PostList) -> Document {
    view! { @use_base [
        "Instrukcio",
        view! {"Instrukcio"},
        None,
        posts,
    ] {
        p { "Nenio estas ĉi tie..." }
        p { "Revenu baldaŭ..." }
    }}
    .into()
}

fn posts_percent<F>(posts: &PostList, predicate: F) -> usize
where
    F: Fn(&Post) -> bool,
{
    posts.iter().filter(|post| predicate(post.get())).count() * 100 / posts.len()
}

pub fn at_list(posts: &PostList) -> Document {
    view! { @use_base [
        "Alia listo",
        view!{},
        None,
        posts,
    ] {
        div ."big-list" {
            div ."stats" {
                table {
                    [:where
                        let percent_new = posts_percent(posts, |post| !post.is_old);
                        let percent_transcribed = posts_percent(posts, |post| post.transcript.is_some());
                    {
                        tr {
                            td { "Novaj:" }
                            td { b { [percent_new] "%" } }
                        }
                        tr {
                            td { "Transskribitaj:" }
                            td { b { [percent_transcribed] "%" } }
                        }
                    }]
                }
            }
            div ."legend" {
                p {
                    [icons::GOOD]       ": Plej bona"     br/
                    [icons::TRANSCRIPT] ": Transskribita" br/
                    [icons::OLD]        ": Estas olda"    br/
                    [icons::NOT_OLD]    ": Estas nova"    br/
                    [icons::REVISED]    ": Retradukita"
                }
            }
            table ."graph" {
                [:for post in posts.into_iter().rev() { [:where let post = post.get(); {
                        tr {
                            td { [:if post.props.good { [icons::GOOD] }] }
                            td { a [href=url!(post.index()), title=post.title] {
                                [:if post.is_sunday
                                    { b  { [&post.index()] } }
                                    else { [&post.index()] }
                                ]
                            }}
                            td { [:if post.transcript.is_some() { [icons::TRANSCRIPT] }] }
                            td { [:if post.is_old { [icons::OLD] } else { [icons::NOT_OLD] }] }
                            td { [:for _ in 0..post.version { span { [icons::REVISED] } }] }
                        }
                    }]
                }]
            }
        }

    }}
    .into()
}

pub fn at_404(posts: &PostList) -> Document {
    view! { @use_base [
        "404",
        view! { "Paĝo ne trovita!" },
        None,
        &posts,
    ] {
        center {
            "404 - Not found"
        }
    }}
    .into()
}
