use serde::Serialize;
use std::{fs, path::Path};

const DIR: &str = "static/posts";

#[derive(Serialize)]
pub struct PostEntry {
    pub post: Post,
    pub prev: Option<Post>,
    pub next: Option<Post>,
}

#[derive(Clone, Serialize)]
pub struct Post {
    pub index: String,
    pub title: String,
    pub errata: Errata,
    pub props: Props,
    pub date: String,
    pub sunday: bool,
}

#[derive(Default, Clone, Serialize)]
pub struct Errata(pub Vec<(String, String)>);

#[derive(Default, Clone, Copy, Serialize)]
pub struct Props {
    pub revised: bool,
    pub nogarfield: bool,
    pub notext: bool,
    pub good: bool,
    pub earsback: bool,
}

pub fn parse_posts() -> Vec<PostEntry> {
    let mut posts = Vec::new();

    let mut folders: Vec<_> = fs::read_dir(DIR)
        .expect("[IO fail] reading whole directory")
        .flatten()
        .collect();

    // Sort based on folder name
    folders.sort_by_key(|folder| {
        let path = folder.path();
        path.to_string_lossy().to_string()
    });

    // Keep track of existing titles to check for duplicates
    let mut existing_titles = Vec::new();
    // Keep track of existing dates to check for duplicates
    let mut existing_dates = Vec::new();

    for folder in folders {
        let path = folder.path();
        let path = path.to_string_lossy().to_string();

        let index = folder.file_name();
        let index = index.to_string_lossy().to_string();

        let index_number: usize = index.parse().expect("Index is not a number [{index}]");
        assert_eq!(index.len(), 4, "Index is not 4 digits [{index}]");
        assert_eq!(
            index_number,
            posts.len(),
            "Unexpected index number [{index}]"
        );

        let title = format!("{path}/title");
        assert!(
            Path::new(&title).exists(),
            "Title file does not exist [{index}]"
        );
        let title = fs::read_to_string(&title)
            .expect("[IO fail] reading title file")
            .trim()
            .to_string();

        if !title.starts_with("Garfildo ") {
            panic!("Title of {index} does not start with Garfildo [{index}]");
        }

        assert!(
            !existing_titles.contains(&title),
            "Multiple posts have '{title}' as title [{index}]"
        );
        existing_titles.push(title.clone());

        let errata = format!("{path}/errata");
        let errata = if Path::new(&errata).exists() {
            let file = fs::read_to_string(&errata).expect("[IO fail] reading errata file");
            parse_errata(file).expect(&format!("Failed to parse errrata file [{index}]"))
        } else {
            Errata::default()
        };

        let props = format!("{path}/props");
        let props = if Path::new(&props).exists() {
            let file = fs::read_to_string(&props).expect("[IO fail] reading props file");
            parse_props(file).expect(&format!("Failed to parse props file [{index}]"))
        } else {
            Props::default()
        };

        let date = format!("{path}/date");
        assert!(
            Path::new(&date).exists(),
            "Title file does not exist [{index}]"
        );
        let date = fs::read_to_string(&date)
            .expect("[IO fail] reading date file")
            .trim()
            .to_string();
        //TODO: check if date is valid
        assert!(
            !existing_dates.contains(&date),
            "Multiple posts have '{date}' as date [{index}]"
        );
        existing_dates.push(date.clone());

        assert!(
            Path::new(&format!("{path}/esperanto.png")).exists(),
            "Missing Esperanto image [{index}]"
        );
        assert!(
            Path::new(&format!("{path}/english.png")).exists(),
            "Missing English image [{index}]"
        );

        let index_int = index
            .parse::<u32>()
            .expect("Index is not an integer [{index}]");
        let sunday = (index_int + 1) % 7 == 0;

        posts.push(Post {
            index,
            title,
            errata,
            props,
            date,
            sunday,
        });
    }

    posts.reverse();

    // Include previous and next posts with each post
    get_neighbors(posts)
}

fn parse_errata(file: String) -> Result<Errata, &'static str> {
    if file.trim().is_empty() {
        return Err("Empty errata file");
    }

    let mut entries = Vec::new();

    for line in file.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let mut split = line.split(">>");

        let Some(bad) = split.next() else {
            return Err("Missing incorrect phrase");
        };
        let Some(good) = split.next() else {
            return Err("Missing correct phrase");
        };

        entries.push((bad.trim().to_string(), good.trim().to_string()));
    }

    Ok(Errata(entries))
}

fn parse_props(file: String) -> Result<Props, String> {
    if file.trim().is_empty() {
        return Err(format!("Empty properties file"));
    }

    let mut props = Props::default();

    for line in file.lines() {
        match line.trim() {
            "good" => props.good = true,
            "revised" => props.revised = true,
            "nogarfield" => props.nogarfield = true,
            "notext" => props.notext = true,
            "earsback" => props.earsback = true,

            "" => continue,
            prop => return Err(format!("Unknown property '{prop}'")),
        }
    }

    Ok(props)
}

fn get_neighbors(posts: Vec<Post>) -> Vec<PostEntry> {
    let mut neighbors = Vec::new();

    for (i, post) in posts.iter().enumerate() {
        let next = match i.checked_sub(1) {
            Some(index) => posts.get(index).cloned(),
            None => None,
        };

        let prev = posts.get(i + 1).cloned();

        neighbors.push(PostEntry {
            post: post.to_owned(),
            prev,
            next,
        });
    }

    neighbors
}
