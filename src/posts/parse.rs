use anyhow::{bail, Context, Result};
use std::{
    collections::HashMap,
    fs::{self, DirEntry},
    path::Path,
};

use super::{Errata, Index, Post, PostList, Props, Special, Transcript};

pub fn parse_posts() -> Result<PostList> {
    let dir_posts = Path::new("static/posts");
    let dir_old = Path::new("static/old");

    let mut folders: Vec<_> = fs::read_dir(dir_posts)?.flatten().collect();
    folders.sort_by_key(|folder| {
        let path = folder.path();
        path.to_string_lossy().to_string()
    });

    let old_versions = get_old_versions(dir_old).with_context(|| "Failed to parse old posts")?;

    let mut posts = Vec::new();
    let mut existing_titles = Vec::new();
    let mut existing_dates = Vec::new();

    for (index, folder) in folders.into_iter().enumerate() {
        let index = Index(index);

        let version = old_versions
            .get(&index.as_int())
            .map(|version| *version + 1)
            .unwrap_or(0);

        let post = parse_post(index, folder, version, &existing_titles, &existing_dates)
            .with_context(|| format!("For post [{}]", index))?;

        existing_titles.push(post.title.clone());
        existing_dates.push(post.date.clone());

        posts.push(post);
    }

    posts.reverse();
    Ok(PostList::new(posts))
}

fn parse_post(
    index: Index,
    folder: DirEntry,
    version: u32,
    existing_titles: &[String],
    existing_dates: &[String],
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
    let title = fs::read_to_string(path)?.trim().to_string();
    if !title.starts_with("Garfildo ") {
        bail!("Title of {index} does not start with 'Garfildo'");
    }
    if existing_titles.contains(&title) {
        bail!("Multiple posts have '{title}' as title");
    }

    let path = folder_path.join(Path::new("date"));
    let date = fs::read_to_string(path)?.trim().to_string();
    if !is_valid_date(&date) {
        bail!("Invalid date `{}`", date);
    }
    if existing_dates.contains(&date) {
        bail!("Multiple posts have '{date}' as date");
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

    let path = folder_path.join(Path::new("errata"));
    let errata = if path.exists() {
        let file = fs::read_to_string(&path)?;
        Errata::try_from(file).with_context(|| "Failed to parse errata file")?
    } else {
        Errata::default()
    };

    let image_path = folder_path.join(Path::new("esperanto.png"));
    if !image_path.exists() {
        bail!("Missing `esperanto.png`");
    }
    if !folder_path.join(Path::new("english.png")).exists() {
        bail!("Missing `english.png`");
    }

    let is_old = !folder_path.join(Path::new("esperanto.svg")).exists();

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
        errata,
        version,
        is_old,
        image_bytes,
    })
}

fn get_old_versions(dir: &Path) -> Result<HashMap<usize, u32>> {
    let mut old_folders: Vec<_> = fs::read_dir(dir)?.flatten().collect();
    old_folders.sort_by_key(|folder| {
        let path = folder.path();
        path.to_string_lossy().to_string()
    });

    let mut old_versions = HashMap::new();
    for folder in old_folders {
        let name = folder.file_name();
        let name = name.to_string_lossy().to_string();
        let mut split = name.split(':');

        let index: Option<usize> = split.next().and_then(|string| string.parse().ok());
        let version: Option<u32> = split.next().and_then(|string| string.parse().ok());
        let Some((index, version)) = index.zip(version) else {
            bail!("For folder `{}`", folder.path().display());
        };

        let expected_version = match old_versions.get(&index) {
            Some(prev) => prev + 1,
            None => 0,
        };

        if version != expected_version {
            bail!(
                "Revised post version out of order. Expected `:{}`, found `:{}`",
                expected_version,
                version
            );
        }

        old_versions.insert(index, version);
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
                "notext" => props.notext = true,
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
            "kristnasko" => Self::Christmas,
            "haloveno" => Self::Halloween,
            file => bail!("Not a special occasion `{}`", file),
        })
    }
}

impl TryFrom<String> for Errata {
    type Error = anyhow::Error;
    fn try_from(file: String) -> Result<Self> {
        if file.trim().is_empty() {
            bail!("Empty errata file".to_string());
        }

        let mut items = Vec::new();
        for line in file.lines() {
            if line.trim().is_empty() {
                continue;
            }

            let mut split = line.split(">>");

            let Some(bad) = split.next() else {
                bail!("Missing incorrect phrase".to_string());
            };
            let Some(good) = split.next() else {
                bail!(format!("Missing correct phrase for '{}'", bad));
            };

            items.push((bad.trim().to_string(), good.trim().to_string()));
        }

        Ok(Errata { items })
    }
}
