use async_trait::async_trait;

pub mod piratebay;

pub struct Query {
    pub title: String,
    pub year: String,
    pub season: i32,
    pub episode: i32,
    pub resolution: Resolution,
    pub language: String,
    pub media_type: MediaType,
}

pub trait QueryResolver {
    fn resolve(&self, query: &Query) -> String;
}

pub enum MediaType {
    Movie,
    TV,
}

pub enum Resolution {
    SD,
    HD,
    FHD,
    UHD,
}

pub enum Language {
    English,
    Spanish,
    French,
    German,
    Italian,
    Portuguese,
    Russian,
    Japanese,
    Korean,
    Chinese,
}

#[derive(Debug)]
pub struct Magnet(pub String);

#[derive(Debug)]
pub enum Media {
    Movie {
        title: String,
        size: String,
        description: String,
        magnet: Magnet,
        seeders: i32,
        leechers: i32,
    },
    TV {
        title: String,
        size: String,
        description: String,
        magnet: Magnet,
        seeders: i32,
        leechers: i32,
    },
}

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[async_trait]
pub trait Explorer {
    async fn new(url: String) -> Self;

    async fn search(&self, query: Query) -> Result<Vec<Media>>;
}