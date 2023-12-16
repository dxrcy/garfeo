pub mod transcript;
use transcript::Transcript;

use std::{collections::HashMap, fs, path::Path};

const DIR: &str = "static/posts";
const DIR_OLD: &str = "static/old";

pub struct PostEntry {
    pub post: Post,
    pub prev: Option<Post>,
    pub next: Option<Post>,
}

#[derive(Clone)]
pub struct Post {
    pub index: String,
    pub title: String,
    pub errata: Errata,
    pub props: Props,
    pub date: String,
    pub sunday: bool,
    pub image_bytes: u64,
    pub version: u32,
    pub transcript: Option<Transcript>,
    pub special: Option<Special>,
}

#[derive(Default, Clone)]
pub struct Errata(pub Vec<(String, String)>);

#[derive(Default, Clone, Copy)]
pub struct Props {
    pub nogarfield: bool,
    pub notext: bool,
    pub good: bool,
    pub earsback: bool,
}

#[derive(Clone, Copy)]
pub enum Special {
    Christmas,
    Halloween,
}

pub fn parse_posts() -> Result<Vec<PostEntry>, String> {
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

    let mut errata_count = 0;

    let mut old_folders: Vec<_> = fs::read_dir(DIR_OLD)
        .expect("[IO fail] reading old posts directory")
        .flatten()
        .collect();
    // Sort based on folder name
    old_folders.sort_by_key(|folder| {
        let path = folder.path();
        path.to_string_lossy().to_string()
    });

    let mut old_versions = HashMap::<String, u32>::new();
    for folder in old_folders {
        let name = folder.file_name();
        let name = name.to_string_lossy().to_string();
        let mut split = name.split(':');

        let index = split.next().expect("Old post should have index");
        let version = split
            .next()
            .expect("Old post should have version")
            .parse::<u32>()
            .expect("Old post version should be number");

        let expected = match old_versions.get(index) {
            Some(prev) => prev + 1,
            None => 0,
        };

        if version != expected {
            return Err(format!(
                "Revised post version out of order. Should be `:{}`, not `:{}`",
                expected, version
            ));
        }

        old_versions.insert(index.to_string(), version);
    }

    for folder in folders {
        let path = folder.path();
        let path = path.to_string_lossy().to_string();

        let index = folder.file_name();
        let index = index.to_string_lossy().to_string();

        let Ok(index_number) = index.parse::<usize>() else {
            return Err(format!("Index is not a number [{index}]"));
        };
        if index.len() != 4 {
            return Err(format!("Index is not 4 digits [{index}]"));
        }
        if index_number != posts.len() {
            return Err(format!("Unexpected index number [{index}]"));
        }

        let title = format!("{path}/title");

        if !Path::new(&title).exists() {
            return Err(format!("Title file does not exist [{index}]"));
        }
        let title = fs::read_to_string(&title)
            .expect("[IO fail] reading title file")
            .trim()
            .to_string();

        if !title.starts_with("Garfildo ") {
            return Err(format!(
                "Title of {index} does not start with 'Garfildo' [{index}]"
            ));
        }
        if existing_titles.contains(&title) {
            return Err(format!("Multiple posts have '{title}' as title [{index}]"));
        }

        existing_titles.push(title.clone());

        let errata = format!("{path}/errata");
        let errata = if Path::new(&errata).exists() {
            let file = fs::read_to_string(&errata).expect("[IO fail] reading errata file");
            println!("\x1b[33mwarning: post {index} has an error\x1b[0m");
            errata_count += 1;
            match parse_errata(file) {
                Ok(errata) => errata,
                Err(err) => {
                    return Err(format!("Failed to parse errrata file [{index}] - {}", err))
                }
            }
        } else {
            Errata::default()
        };

        let props = format!("{path}/props");
        let props = if Path::new(&props).exists() {
            let file = fs::read_to_string(&props).expect("[IO fail] reading props file");
            match parse_props(file) {
                Ok(props) => props,
                Err(err) => return Err(format!("Failed to parse props file [{index}] - {}", err)),
            }
        } else {
            Props::default()
        };

        let date = format!("{path}/date");
        if !Path::new(&date).exists() {
            return Err(format!("Date file does not exist [{index}]"));
        }
        let date = fs::read_to_string(&date)
            .expect("[IO fail] reading date file")
            .trim()
            .to_string();
        //TODO: check if date is valid
        if existing_dates.contains(&date) {
            return Err(format!("Multiple posts have '{date}' as date [{index}]"));
        }
        existing_dates.push(date.clone());

        let esperanto = format!("{path}/esperanto.png");
        if !Path::new(&esperanto).exists() {
            return Err(format!("Missing Esperanto image [{index}]"));
        }
        let image_bytes = fs::metadata(esperanto)
            .expect("[IO fail] reading esperanto image metadata")
            .len();

        if !Path::new(&format!("{path}/english.png")).exists() {
            return Err(format!("Missing English image [{index}]"));
        }

        let Ok(index_int) = index.parse::<u32>() else {
            return Err(format!("Index is not an integer [{index}]"));
        };
        let sunday = (index_int + 1) % 7 == 0;

        let version = match old_versions.get(&index) {
            Some(prev) => prev + 1,
            None => 0,
        };

        let transcript = format!("{path}/transcript");
        let transcript = if Path::new(&transcript).exists() {
            let file = fs::read_to_string(&transcript).expect("[IO fail] reading transcript file");
            match Transcript::from_file(&file) {
                Ok(props) => Some(props),
                Err(err) => {
                    return Err(format!(
                        "Failed to parse transcript file [{index}] - {}",
                        err
                    ))
                }
            }
        } else {
            None
        };

        let special = format!("{path}/special");
        let special = if Path::new(&special).exists() {
            let file = fs::read_to_string(&special).expect("[IO fail] reading special file");
            match Special::from_file(&file) {
                Ok(props) => Some(props),
                Err(err) => {
                    return Err(format!("Failed to parse special file [{index}] - {}", err))
                }
            }
        } else {
            None
        };

        posts.push(Post {
            index,
            title,
            errata,
            props,
            date,
            sunday,
            image_bytes,
            version,
            transcript,
            special,
        });
    }

    if errata_count > 0 {
        println!("\x1b[33mwarning: {} posts have errors\x1b[0m", errata_count);
    }

    posts.reverse();

    // Include previous and next posts with each post
    Ok(get_neighbors(posts))
}

fn parse_errata(file: String) -> Result<Errata, String> {
    if file.trim().is_empty() {
        return Err("Empty errata file".to_string());
    }

    let mut entries = Vec::new();

    for line in file.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let mut split = line.split(">>");

        let Some(bad) = split.next() else {
            return Err("Missing incorrect phrase".to_string());
        };
        let Some(good) = split.next() else {
            return Err(format!("Missing correct phrase for '{}'", bad));
        };

        entries.push((bad.trim().to_string(), good.trim().to_string()));
    }

    Ok(Errata(entries))
}

fn parse_props(file: String) -> Result<Props, String> {
    if file.trim().is_empty() {
        return Err("Empty properties file".to_string());
    }

    let mut props = Props::default();

    for line in file.lines() {
        match line.trim() {
            "good" => props.good = true,
            "nogarfield" => props.nogarfield = true,
            "notext" => props.notext = true,
            "earsback" => props.earsback = true,
            "" => continue,
            prop => return Err(format!("Unknown property '{prop}'")),
        }
    }

    Ok(props)
}

impl Special {
    pub fn from_file(file: &str) -> Result<Self, String> {
        Ok(match file.trim() {
            "kristnasko" => Self::Christmas,
            "haloveno" => Self::Halloween,
            file => return Err(format!("not a special occasion `{}`", file)),
        })
    }
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

impl Post {
    pub fn to_json(&self) -> String {
        let Post {
            index,
            title,
            date,
            version,
            errata,
            sunday,
            image_bytes,
            props:
                Props {
                    nogarfield,
                    notext,
                    good,
                    earsback,
                },
            transcript: _,
            special: _,
        } = self;

        let errata = if errata.0.is_empty() {
            "[]".to_string()
        } else {
            format!(
                r#"[
        {}
    ]"#,
                errata
                    .0
                    .iter()
                    .map(|(old, new)| format!(r#"["{}", "{}"]"#, old, new))
                    .collect::<Vec<_>>()
                    .join(",\n        ")
            )
        };

        format!(
            r#"{{
    "index": "{index}",
    "title": "{title}",
    "date": "{date}",
    "version": {version},
    "sunday": "{sunday}",
    "image_bytes": {image_bytes},
    "errata": {errata},
    "props": {{
        "nogarfield": {nogarfield},
        "notext": {notext},
        "good": {good},
        "earsback": {earsback}
    }}
}}"#,
        )
    }
}

impl PostEntry {
    pub fn to_json(&self) -> String {
        let Self { post, prev, next } = self;
        let Post {
            index,
            title,
            date,
            version,
            errata,
            sunday,
            image_bytes,
            props:
                Props {
                    nogarfield,
                    notext,
                    good,
                    earsback,
                },
            transcript: _,
            special: _,
        } = post;

        let prev = match &prev {
            Some(post) => format!("{:?}", post.index),
            None => "null".to_string(),
        };
        let next = match &next {
            Some(post) => format!("{:?}", post.index),
            None => "null".to_string(),
        };

        let errata = if errata.0.is_empty() {
            "[]".to_string()
        } else {
            format!(
                r#"[
        {}
    ]"#,
                errata
                    .0
                    .iter()
                    .map(|(old, new)| format!(r#"["{}", "{}"]"#, old, new))
                    .collect::<Vec<_>>()
                    .join(",\n        ")
            )
        };

        format!(
            r#"{{
    "index": "{index}",
    "prev": {prev},
    "next": {next},
    "title": "{title}",
    "date": "{date}",
    "version": {version},
    "sunday": "{sunday}",
    "image_bytes": {image_bytes},
    "errata": {errata},
    "props": {{
        "nogarfield": {nogarfield},
        "notext": {notext},
        "good": {good},
        "earsback": {earsback}
    }}
}}"#,
        )
    }
}
