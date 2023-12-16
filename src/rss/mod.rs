use crate::posts::PostList;

pub fn generate_rss(posts: PostList) -> String {
    let title = "Garfildo Esperanta";
    let url = "https://darccyy.github.io/garfeo";
    let description = "Legu 500+ bildstrioj de Garfildo, tradukitaj en Esperanton!";

    format!(
        include_str!("file.rss"),
        title = title,
        url = url,
        description = description,
        last_index = posts.first().index(),
        items = posts
            .into_iter()
            .map(|post| {
                let post = post.get();
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
