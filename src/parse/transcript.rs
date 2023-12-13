#[derive(Clone, Debug)]
pub enum Transcript {
    Normal([Panel; 3]),
    Sunday([Panel; 7]),
}

// #[derive(Clone, Debug)]
// pub struct Transcript {
//     pub panels: [Panel; 3],
// }

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
    Character(String),
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

        let transcript = match panels.len() {
            3 => Transcript::Normal(panels.try_into().expect("panels should convert to array")),
            7 => Transcript::Sunday(panels.try_into().expect("panels should convert to array")),
            _ => return Err(format!("must be 3 or 7 panels")),
        };

        Ok(transcript)
    }

    pub fn panels(&self) -> &[Panel] {
        match self {
            Self::Normal(panels) => panels,
            Self::Sunday(panels) => panels,
        }
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
            let speaker = Speaker::from_string(line)?;

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
    fn from_string(string: &str) -> Result<Self, String> {
        if !string.ends_with(':') {
            return Ok(match string.to_lowercase().as_str() {
                "[sono]" => Self::Sound,
                "[skribo]" => Self::Text,
                _ => return Err(format!("not a valid speaker `{}`", string)),
            });
        }

        let string = remove_last_char(&string).to_lowercase();
        Ok(Self::Character(string))
    }
}
