#[derive(Clone, Debug)]
pub struct Transcript {
    pub panels: [Panel; 3],
}

#[derive(Clone, Debug)]
pub struct Panel {
    pub texts: Vec<Text>,
}

#[derive(Clone, Debug)]
pub struct Text {
    pub speaker: Speaker,
    pub text: String,
}

#[derive(Clone, Debug)]
pub enum Speaker {
    Sound,
    Text,
    Character(Character),
}

#[derive(Clone, Debug, Copy)]
pub enum Character {
    Garfield,
    Jon,
    Liz,
    Odie,
    Nermal,
    Arlene,
}

impl Transcript {
    pub fn from_file(file: &str) -> Result<Transcript, String> {
        let mut panels = Vec::new();
        let mut panel_lines = Vec::new();

        for line in file.lines() {
            let line = line.trim();

            if line == "---" {
                panels.push(panel_lines);
                panel_lines = Vec::new();
            } else {
                panel_lines.push(line);
            }
        }
        panels.push(panel_lines);

        let panels: Vec<_> =
            extract_first_error(panels.into_iter().map(Panel::from_lines))?.collect();
        let Ok(panels) = panels.try_into() else {
            return Err(format!("must be 3 panels"));
        };

        println!("{:#?}", panels);

        Ok(Transcript { panels })
    }
}

fn extract_first_error<T, E>(
    it: impl Iterator<Item = Result<T, E>>,
) -> Result<impl Iterator<Item = T>, E> {
    let mut vec = Vec::new();
    for item in it {
        vec.push(item?);
    }
    Ok(vec.into_iter())
}

impl Panel {
    fn from_lines(lines: Vec<&str>) -> Result<Panel, String> {
        let mut lines = lines.into_iter();
        let mut texts = Vec::new();

        while let Some(line) = lines.next() {
            if !line.ends_with(':') {
                return Err(format!("expected character definition"));
            }
            let speaker = remove_last_char(line);
            let Some(speaker) = Speaker::from_string(speaker) else {
                return Err(format!("unknown speaker `{}`", speaker));
            };

            let Some(text) = lines.next() else {
                return Err(format!("expected text line after `{}`", line));
            };
            let text = text.to_string();

            texts.push(Text { speaker, text });
        }

        Ok(Panel { texts })
    }
}

fn remove_last_char(string: &str) -> &str {
    let mut chars = string.chars();
    chars.next_back();
    chars.as_str()
}

impl Speaker {
    fn from_string(string: &str) -> Option<Self> {
        Some(match string.to_lowercase().as_str() {
            "[sound]" => Self::Sound,
            "[text]" => Self::Text,
            "garfield" => Self::Character(Character::Garfield),
            "jon" => Self::Character(Character::Jon),
            "odie" => Self::Character(Character::Odie),
            "liz" => Self::Character(Character::Liz),
            "nermal" => Self::Character(Character::Nermal),
            "arlene" => Self::Character(Character::Arlene),
            _ => return None,
        })
    }
}
