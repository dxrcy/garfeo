use anyhow::{bail, Result};

#[derive(Debug)]
pub enum Transcript {
    Normal([Panel; 3]),
    Sunday([Panel; 7]),
}

#[derive(Debug)]
pub struct Panel {
    pub lines: Vec<Line>,
}

#[derive(Debug)]
pub struct Line {
    pub speaker: Speaker,
    pub text: String,
}

#[derive(Debug)]
pub enum Speaker {
    Sound,
    Text,
    Character { name: String, uncommon: bool },
}

impl Transcript {
    pub fn panels(&self) -> &[Panel] {
        match self {
            Self::Normal(panels) => panels,
            Self::Sunday(panels) => panels,
        }
    }
}

impl TryFrom<String> for Transcript {
    type Error = anyhow::Error;
    fn try_from(file: String) -> Result<Self> {
        let mut panels_lines = Vec::new();
        let mut lines = Vec::new();

        for file_line in file.lines() {
            let file_line = file_line.trim();
            if file_line.is_empty() {
                continue;
            }
            if file_line == "---" {
                panels_lines.push(lines);
                lines = Vec::new();
            } else {
                lines.push(file_line);
            }
        }
        panels_lines.push(lines);

        if panels_lines.is_empty() {
            bail!("Empty file");
        }

        let mut panels = Vec::new();

        for lines in panels_lines {
            panels.push(Panel::try_from(lines)?);
        }

        let transcript = match panels.len() {
            3 => Transcript::Normal(panels.try_into().expect("panels should convert to array")),
            7 => Transcript::Sunday(panels.try_into().expect("panels should convert to array")),
            _ => bail!("Must contain exactly 3 OR 7 panels"),
        };

        Ok(transcript)
    }
}

impl TryFrom<Vec<&str>> for Panel {
    type Error = anyhow::Error;
    fn try_from(strings: Vec<&str>) -> Result<Self> {
        let mut strings = strings.into_iter();
        let mut lines = Vec::new();

        while let Some(string) = strings.next() {
            let speaker = Speaker::try_from(string)?;

            let Some(text) = strings.next() else {
                bail!("expected text line after `{}`", string);
            };
            let text = text.to_string();

            lines.push(Line { speaker, text });
        }

        Ok(Panel { lines })
    }
}

impl TryFrom<&str> for Speaker {
    type Error = anyhow::Error;
    fn try_from(string: &str) -> Result<Self> {
        if !string.ends_with(':') {
            return Ok(match string.to_lowercase().as_str() {
                "[sono]" => Self::Sound,
                "[skribo]" => Self::Text,
                _ => bail!("Not a valid speaker `{}`", string),
            });
        }

        let name = remove_last_char(string).to_lowercase();
        let uncommon = name.starts_with('~');

        let name = if uncommon {
            remove_first_char(&name).to_string()
        } else {
            name
        };

        Ok(Self::Character { name, uncommon })
    }
}

fn remove_last_char(string: &str) -> &str {
    let mut chars = string.chars();
    chars.next_back();
    chars.as_str()
}
fn remove_first_char(string: &str) -> &str {
    let mut chars = string.chars();
    chars.next();
    chars.as_str()
}
