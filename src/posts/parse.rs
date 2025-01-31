use anyhow::{bail, Context, Result};
use std::{
    collections::{HashMap, HashSet},
    fs::{self, DirEntry},
    path::Path,
};

use super::{Index, Post, PostList, Props, Special, Transcript};

pub fn parse_posts() -> Result<PostList> {
    let dir_posts = Path::new("assets/posts");
    let dir_old = Path::new("assets/old");

    let mut folders: Vec<_> = fs::read_dir(dir_posts)?.flatten().collect();
    folders.sort_by_key(|folder| {
        let path = folder.path();
        path.to_string_lossy().to_string()
    });

    let old_versions = get_old_versions(dir_old).with_context(|| "Failed to parse old posts")?;

    let mut posts = Vec::new();
    let mut existing_titles = HashMap::new();
    let mut existing_dates = HashMap::new();

    for (index, folder) in folders.into_iter().enumerate() {
        let index = Index(index);

        let is_revised = old_versions.contains(&index.as_int());

        let post = parse_post(index, folder, is_revised, &existing_titles, &existing_dates)
            .with_context(|| format!("For post [{}]", index))?;

        existing_titles.insert(post.title.clone(), index);
        existing_dates.insert(post.date.clone(), index);

        posts.push(post);
    }

    // posts.reverse();
    Ok(PostList::new(posts))
}

fn parse_post(
    index: Index,
    folder: DirEntry,
    is_revised: bool,
    existing_titles: &HashMap<String, Index>,
    existing_dates: &HashMap<String, Index>,
) -> Result<Post> {
    let folder_name = folder.file_name();
    let folder_name = folder_name.to_string_lossy();
    let folder_path = folder.path();

    let expected_name = index.to_string();
    if folder_name != expected_name {
        bail!(
            "Invalid folder name. Expected `{}`, found `{}`",
            expected_name,
            folder_name
        );
    }

    let path = folder_path.join(Path::new("title"));
    if !path.exists() {
        bail!("Missing `title` file");
    }
    let title = fs::read_to_string(path)?.trim().to_string();
    if title.is_empty() {
        bail!("Title of {index} is empty")
    }
    if !title.starts_with("Garfildo ") {
        bail!("Title of {index} does not start with 'Garfildo'");
    }
    if let Some(other_index) = existing_titles.get(&title) {
        bail!(
            "Multiple posts have '{title}' as title\nPosts {} and {}",
            other_index,
            index
        );
    }

    let path = folder_path.join(Path::new("date"));
    if !path.exists() {
        bail!("Missing `date` file");
    }
    let date = fs::read_to_string(path)?.trim().to_string();
    if !is_valid_date(&date) {
        bail!("Invalid date `{}`", date);
    }
    if let Some(other_index) = existing_dates.get(&date) {
        bail!(
            "Multiple posts have '{date}' as date\nPosts {} and {}",
            other_index,
            index
        );
    }

    let is_sunday = (index.as_int() + 1) % 7 == 0;

    let path = folder_path.join(Path::new("transcript"));
    let transcript = if path.exists() {
        let file = fs::read_to_string(&path)?;
        Some(Transcript::try_from(file).with_context(|| "Failed to parse transcript file")?)
    } else {
        None
    };

    let path = folder_path.join(Path::new("props"));
    let props = if path.exists() {
        let file = fs::read_to_string(&path)?;
        Props::try_from(file).with_context(|| "Failed to parse props file")?
    } else {
        Props::default()
    };

    let path = folder_path.join(Path::new("special"));
    let special = if path.exists() {
        let file = fs::read_to_string(&path)?;
        Some(Special::try_from(file).with_context(|| "Failed to parse special file")?)
    } else {
        None
    };

    let image_path = folder_path.join(Path::new("esperanto.png"));
    if !image_path.exists() {
        bail!("Missing `esperanto.png` file");
    }
    if !folder_path.join(Path::new("english.png")).exists() {
        bail!("Missing `english.png` file");
    }

    let is_old = !folder_path.join(Path::new("esperanto.svg")).exists();
    let is_simple = !folder_path.join(Path::new("esperanto.xcf")).exists();

    let image_bytes = fs::metadata(image_path)
        .with_context(|| "Reading size of `esperanto.png`")?
        .len();

    Ok(Post {
        index,
        title,
        date,
        is_sunday,
        transcript,
        props,
        special,
        is_revised,
        is_old,
        is_simple,
        image_bytes,
    })
}

fn get_old_versions(dir: &Path) -> Result<HashSet<usize>> {
    let mut old_folders: Vec<_> = fs::read_dir(dir)?.flatten().collect();
    old_folders.sort_by_key(|folder| {
        let path = folder.path();
        path.to_string_lossy().to_string()
    });

    let mut old_versions = HashSet::new();
    for folder in old_folders {
        let name = folder.file_name();
        let name = name.to_string_lossy().to_string();

        let Some(index) = name.parse::<usize>().ok() else {
            bail!("For folder `{}`", folder.path().display());
        };

        old_versions.insert(index);
    }

    Ok(old_versions)
}

fn is_valid_date(date: &str) -> bool {
    let mut split = date.split('-');
    let Some(((year, month), day)) = split.next().zip(split.next()).zip(split.next()) else {
        return false;
    };
    split.next().is_none()
        && year.len() == 4
        && month.len() == 2
        && day.len() == 2
        && year.parse::<u32>().is_ok()
        && month.parse::<u32>().is_ok()
        && day.parse::<u32>().is_ok()
}

impl TryFrom<String> for Props {
    type Error = anyhow::Error;
    fn try_from(file: String) -> Result<Self> {
        if file.trim().is_empty() {
            bail!("Empty props file");
        }

        let mut props = Self::default();
        for line in file.lines() {
            match line.trim() {
                "good" => props.good = true,
                "nogarfield" => props.nogarfield = true,
                "earsback" => props.earsback = true,
                "" => continue,
                prop => bail!("Unknown property '{prop}'"),
            }
        }

        Ok(props)
    }
}

impl TryFrom<String> for Special {
    type Error = anyhow::Error;
    fn try_from(file: String) -> Result<Self> {
        Ok(match file.trim() {
            "haloveno" => Self::Halloween,
            "kristnasko" => Self::Christmas,
            "novjaro" => Self::NewYears,
            file => bail!("Not a special occasion `{}`", file),
        })
    }
}
