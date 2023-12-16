mod parse;
mod structs;
pub mod transcript;

use self::transcript::Transcript;
pub use parse::parse_posts;

pub type PostList = structs::List<Post>;
pub type PostRef<'a> = structs::ItemRef<'a, Post>;

pub use structs::{ListEnds, Neighbors};

#[derive(Debug)]
pub struct Post {
    pub index: Index,
    pub title: String,
    pub date: String,
    pub is_sunday: bool,
    pub transcript: Option<Transcript>,
    pub props: Props,
    pub special: Option<Special>,
    pub errata: Errata,
    pub version: u32,
    pub is_old: bool,
    pub image_bytes: u64,
}

#[derive(Clone, Copy, Debug)]
pub struct Index(usize);

#[derive(Debug, Default)]
pub struct Props {
    pub nogarfield: bool,
    pub notext: bool,
    pub good: bool,
    pub earsback: bool,
}

#[derive(Clone, Copy, Debug)]
pub enum Special {
    Christmas,
    Halloween,
}

#[derive(Debug, Default)]
pub struct Errata {
    pub items: Vec<(String, String)>,
}

impl Post {
    pub fn index(&self) -> String {
        self.index.to_string()
    }
}

impl Index {
    pub fn as_int(&self) -> usize {
        self.0
    }
}

impl std::fmt::Display for Index {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:04}", self.0)
    }
}
