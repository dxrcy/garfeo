use crate::{parse::PostEntry, FirstLast};

pub fn generate_rss(entries: &[PostEntry], first_last: &FirstLast) -> String {
    let title = "Garfildo Esperanta";
    let url = "https://darccyy.github.io/garfeo";
    let description = "Legu 500+ bildstrioj de Garfildo, tradukitaj en Esperanton!";

    format!(
        include_str!("file.rss"),
        title = title,
        url = url,
        description = description,
        last_index = first_last.last.post.index,
        items = entries
            .iter()
            .map(|entry| {
                let post = &entry.post;
                format!(
                    include_str!("item.rss"),
                    url = url,
                    title = post.title,
                    index = post.index,
                    date = post.date,
                    image_bytes = post.image_bytes,
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    )
}
