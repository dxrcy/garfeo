mod at_about;
mod at_post;
mod json;

use ibex::prelude::*;
use std::collections::HashSet;

use crate::posts::{Post, PostList, Special};
use crate::views::{icons, list_item, sentence_case, use_base};

pub use at_about::*;
pub use at_post::*;
pub use json::*;

pub fn at_index(posts: &PostList) -> Document {
    view! { @use_base [
        "",
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
        None,
        posts,
    ] {
        h2 { "Plej bonaj bildstrioj" }

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

fn posts_percent<F>(posts: &PostList, predicate: F) -> usize
where
    F: Fn(&Post) -> bool,
{
    posts.iter().filter(|post| predicate(post.get())).count() * 100 / posts.len()
}

fn posts_names(posts: &PostList) -> [Vec<(String, bool)>; 2] {
    let mut seen = HashSet::new();
    let speakers = posts
        .iter()
        .filter_map(|post| post.get().transcript.clone())
        .flat_map(|transcript| transcript.names())
        .filter(|name| seen.insert(name.clone()));

    let (mut common, mut uncommon): (Vec<_>, Vec<_>) =
        speakers.partition(|(_, uncommon)| !uncommon);
    common.sort();
    uncommon.sort();
    [common, uncommon]
}

pub fn at_list(posts: &PostList) -> Document {
    view! { @use_base [
        "Alia listo",
        None,
        posts,
    ] {
        div ."big-list" {
            div ."stats" {
                table {
                    [:where
                        let percent_new = posts_percent(posts, |post| !post.is_old);
                        let percent_transcribed = posts_percent(posts, |post| post.transcript.is_some());
                        let percent_total = (percent_new + percent_transcribed) / 2;
                    {
                        tr { td/        td { b { [percent_new]         "%" } } td { "Novaj" } }
                        tr { td/        td { b { [percent_transcribed] "%" } } td { "Transskribitaj" } }
                        tr { td { "=" } td { b { i { [percent_total]   "%" }}} td/ }
                    }]
                }
            }
            div ."names" {
                [:where
                    let [names_common, names_uncommon] = posts_names(posts);
                    fn section(title: &str, names: Vec<(String, bool)>) -> View {
                        view!{
                            div {
                                h3 { [title] }
                                p {
                                    [:for (i, (name, _)) in names.into_iter().enumerate() {
                                        [:if i > 0 { ","~ }]
                                        i { [sentence_case(&name, true)] }
                                    }]
                                }
                            }
                        }
                    }
                {
                    @section["Oftaj Nomoj",    names_common]
                    @section["Maloftaj Nomoj", names_uncommon]
                }]
            }
            div ."legend" {
                table {
                    [:where let percent_good = posts_percent(posts, |post| post.props.good); {
                        tr { td { [icons::GOOD] }   td { "Bona" ~ i { "(" [percent_good] "%)" } } } }]
                    tr { td { [icons::TRANSCRIPT] } td { "Transskribita" } }
                    tr { td { [icons::OLD] }        td { "Estas olda" } }
                    tr { td { [icons::NOT_OLD] }    td { "Estas nova" } }
                    tr { td { [icons::REVISED] }    td { "Retradukita" } }
                }
                table {
                    tr { td { [icons::CHRISTMAS] }  td { "Por Kristnasko" } }
                    tr { td { [icons::HALLOWEEN] }  td { "Por Haloveno" } }
                    tr { td { [icons::NEW_YEARS] }  td { "Por Novjaron" } }
                }
            }
            table ."graph" {
                [:for post in posts.into_iter().rev() { [:where let post = post.get(); {
                        tr {
                            td { [:if let Some(special) = post.special { [match special {
                                Special::Halloween => icons::HALLOWEEN,
                                Special::Christmas => icons::CHRISTMAS,
                                Special::NewYears => icons::NEW_YEARS,
                            }] }] }

                            td { [:if post.props.good { [icons::GOOD] }] }

                            td { a [href=url!(post.index()), title=post.title] {
                                [:if post.is_sunday
                                     { b { [&post.index()] } }
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
        None,
        &posts,
    ] {
        h3 { "Paĝo ne trovita!" }
        p {
            "404 - Not found"
        }
    }}
    .into()
}
