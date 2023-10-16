use ibex::prelude::*;

/// until added to ibex crate
pub fn is_local() -> bool {
    std::env::args().nth(1) == Some("local".to_string())
}

/// until added to ibex crate
macro_rules! url {
    () => {{
        url!(@root)
    }};
    ($path:expr) => {{
        format!("{}{}", url!(@root), $path.to_string())
    }};
    (@root) => {
        if features::is_local() { "/" } else { URL_ROOT }
    };
}

/// until added to ibex crate
pub struct Meta<'a> {
    pub url: Option<&'a str>,
    pub title: Option<&'a str>,
    pub desc: Option<&'a str>,
    pub image: Option<&'a str>,
    pub author: Option<&'a str>,
    pub color: Option<&'a str>,
}

/// until added to ibex crate
pub fn use_meta(meta: Meta) -> View {
    view! {
        HEAD {
            meta [charset="utf-8"]/
            meta [name="viewport", content="width=device-width, initial-scale=1"]/

            [if let Some(url) = meta.url { view!{
                meta [name="url",        content=url]/
                meta [property="og:url", content=url]/
            }} else { view! {}}]

            [if let Some(title) = meta.title { view!{
                meta [itemprop="name",     content=title]/
                meta [property="og:title", content=title]/
                meta [name="title",        content=title]/
            }} else { view! {}}]

            [if let Some(desc) = meta.desc{ view!{
                meta [name="description",         content=desc]/
                meta [itemprop="description",     content=desc]/
                meta [property="og:description",  content=desc]/
                meta [name="twitter:description", content=desc]/
            }} else { view! {}}]

            [if let Some(image) = meta.image { view!{
                meta [name="image",         content=image]/
                meta [itemprop="image",     content=image]/
                meta [property="og:image",  content=image]/
                meta [name="twitter:image", content=image]/
            }} else { view! {}}]

            [if let Some(author) = meta.author { view!{
                meta [name="author", content=author]/
            }} else { view! {}}]

            [if let Some(color) = meta.color { view!{
                meta [name="theme-color", content=color]/
            }} else { view! {}}]
        }
    }
}
