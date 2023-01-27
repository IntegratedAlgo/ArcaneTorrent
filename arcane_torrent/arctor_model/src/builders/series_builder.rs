use time::Date;

use crate::{entities::Language, Series};

use super::MediaBuilder;

pub trait SeriesBuilder : MediaBuilder {
    fn add_season_id(&mut self, season_id: u32) -> &mut Self;
    fn with_season_ids(&mut self, season_ids: Vec<u32>) -> &mut Self;
}

#[derive(Default)]
pub struct SeriesBuilderImpl {
    tvdb_id: Option<u32>,
    title: Option<String>,
    description: Option<String>,
    release_date: Option<Date>,
    genres: Option<Vec<String>>,
    language: Option<Language>,
    season_tvdb_ids: Vec<u32>
}

impl MediaBuilder for SeriesBuilderImpl {
    type OutputType = Series;

    fn set_tvdb_id(&mut self, tvdb_id: u32) -> &mut Self {
        self.tvdb_id = Some(tvdb_id);
        self
    }

    fn set_title(&mut self, title: String) -> &mut Self {
        self.title = Some(title);
        self
    }

    fn set_description(&mut self, description: String) -> &mut Self {
        self.description = Some(description);
        self
    }

    fn set_release_date(&mut self, release_date: Date) -> &mut Self {
        self.release_date = Some(release_date);
        self
    }

    fn set_language(&mut self, language: Language) -> &mut Self {
        self.language = Some(language);
        self
    }
    
    fn with_genres(&mut self, genres: Vec<String>) -> &mut Self {
        self.genres = Some(genres);
        self    
    }

    fn build(self) -> Self::OutputType {
        Series::new(
            self.tvdb_id.expect("TVDB ID is required"),
            self.title.expect("Title is required"),
            self.description.expect("Description is required"),
            self.release_date.expect("Release date is required"),
            self.language.expect("Language is required"),
            self.genres.expect("Genres are required"),
            self.season_tvdb_ids
        )
    }
}

impl SeriesBuilder for SeriesBuilderImpl {
    fn add_season_id(&mut self, season_id: u32) -> &mut Self {
        self.season_tvdb_ids.push(season_id);
        self
    }

    fn with_season_ids(&mut self, season_ids: Vec<u32>) -> &mut Self {
        self.season_tvdb_ids = season_ids;
        self
    }
}