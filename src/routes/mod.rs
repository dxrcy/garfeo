mod at_post;
mod json;

use ibex::prelude::*;

use crate::posts::PostList;
use crate::views::{list_item, use_base, STAR};
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
            [:for post in posts {
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
        ol ."post-list" [
            reversed=true,
            start=posts.first().index,
        ] {
            [:for post in posts {
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

pub fn at_list(posts: &PostList) -> Document {
    view! { @use_base [
        "Alia listo",
        view!{},
        None,
        posts,
    ] {
        table ."graph" {
            [:for post in posts { [:where let post = post.get(); {
                    tr {
                        td { [:if post.props.good { [STAR] }] }
                        td { a [href=url!(post.index()), title=post.title] {
                            [:if post.is_sunday
                                { b  { [&post.index()] } }
                                else { [&post.index()] }
                            ]
                        }}
                        td { [:for _ in 0..post.version { span { "🟥" } }] }
                    }
                }]
            }]
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
