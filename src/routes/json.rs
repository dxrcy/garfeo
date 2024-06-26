use crate::posts::transcript::Speaker;
use crate::posts::{PostList, PostRef, Props, Special};

pub fn json_index(posts: &PostList) -> String {
    format!(
        r#"[{}]"#,
        posts
            .into_iter()
            .map(json_post)
            .collect::<Vec<_>>()
            .join(",")
    )
}

#[allow(clippy::format_in_format_args)]
pub fn json_post(post_ref: PostRef) -> String {
    let post = post_ref.get();
    format!(
        concat!(
            "{{",
            "\"previous\":{prev},",
            "\"next\":{next},",
            "\"index\":\"{index}\",",
            "\"title\":\"{title}\",",
            "\"date\":\"{date}\",",
            "\"is_sunday\":{is_sunday},",
            "\"transcript\":{transcript},",
            "\"props\":{props},",
            "\"special\":{special},",
            "\"is_revised\":{is_revised},",
            "\"is_old\":{is_old},",
            "\"image_bytes\":{image_bytes}",
            "}}"
        ),
        index = post.index(),
        prev = match post_ref.prev() {
            None => "null".to_string(),
            Some(post) => format!("{:?}", post.index()),
        },
        next = match post_ref.next() {
            None => "null".to_string(),
            Some(post) => format!("{:?}", post.index()),
        },
        title = post.title,
        date = post.date,
        is_sunday = post.is_sunday,
        transcript = match &post.transcript {
            None => "null".to_string(),
            Some(transcript) => format!(
                "[{}]",
                transcript
                    .panels()
                    .iter()
                    .map(|panel| format!(
                        "[{}]",
                        panel
                            .lines
                            .iter()
                            .map(|line| format!(
                                "[\"{}\",\"{}\"]",
                                match &line.speaker {
                                    Speaker::Text => "[skribo]".to_string(),
                                    Speaker::Sound => "[sono]".to_string(),
                                    Speaker::Character { name, .. } => name.to_uppercase(),
                                },
                                line.text
                            ))
                            .collect::<Vec<_>>()
                            .join(",")
                    ))
                    .collect::<Vec<_>>()
                    .join(",")
            ),
        },
        props = {
            let Props {
                nogarfield,
                good,
                earsback,
            } = post.props;
            format!(
                "{{\"nogarfield\":{nogarfield},\
                    \"good\":{good},\
                    \"earsback\":{earsback}}}"
            )
        },
        special = match post.special {
            None => "null",
            Some(Special::Halloween) => "\"haloveno\"",
            Some(Special::Christmas) => "\"kristnasko\"",
            Some(Special::NewYears) => "\"novjaro\"",
        },
        is_revised = post.is_revised,
        is_old = post.is_old,
        image_bytes = post.image_bytes,
    )
}
