use time::Date;

use super::types::Language;

#[derive(Debug, Clone)]
pub struct Series {
    pub tvdb_id: u32,
    pub title: String,
    pub description: String,
    pub release_date: Date,
    pub language: Language,
    pub genres: Vec<String>,
    pub season_tvdb_ids: Vec<u32>
}

impl Series {
    pub fn new(
        tvdb_id: u32,
        title: String,
        description: String,
        release_date: Date,
        language: Language,
        genres: Vec<String>,
        season_tvdb_ids: Vec<u32>
    ) -> Self {
        Self {
            tvdb_id,
            title,
            description,
            release_date,
            language,
            genres,
            season_tvdb_ids
        }
    }
}