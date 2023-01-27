use time::Date;

use super::{types::Language };

#[derive(Debug, Clone)]
pub struct Movie {
    pub tvdb_id: u32,
    pub title: Option<String>,
    pub description: Option<String>,
    pub duration_in_minutes: Option<u32>,
    pub genres: Option<Vec<String>>,
    pub language: Option<Language>,
}

impl Movie {
    pub fn new(
        tvdb_id: u32,
        title: String,
        description: String,
        release_date: Date,
        duration_in_minutes: u32,
        genres: Vec<String>,
        language: Language,
    ) -> Self {
        Self {
            tvdb_id: tvdb_id,
            title: Some(title),
            description: Some(description),
            duration_in_minutes: Some(duration_in_minutes),
            genres: Some(genres),
            language: Some(language),
        }
    }
}