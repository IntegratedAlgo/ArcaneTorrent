use async_trait::async_trait;
use crate::model::{self, Resolution, MediaType, Media};

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
    fn resolve(&self, query: &Query, page: u32) -> String;
}

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[async_trait]
pub trait Explorer {
    async fn new(url: String) -> Self;

    async fn search(&self, query: Query, page_depth: u32) -> Result<Vec<Media>>;
}