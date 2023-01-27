use time::Date;

use crate::{entities::Language, Movie};

use super::MediaBuilder;

pub trait MovieBuilder : MediaBuilder {
    fn set_duration_in_minutes(&mut self, duration_in_minutes: u32) -> &mut Self;
}

#[derive(Default)]
pub struct MovieBuilderImpl {
    tvdb_id: Option<u32>,
    title: Option<String>,
    description: Option<String>,
    release_date: Option<Date>,
    duration_in_minutes: Option<u32>,
    genres: Option<Vec<String>>,
    language: Option<Language>,
}

impl MovieBuilderImpl {
    pub fn new() -> Self {
        Self::default()
    }
}

impl MediaBuilder for MovieBuilderImpl {
    type OutputType = Movie;

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
        Movie::new(
            self.tvdb_id.expect("TVDB ID is required"),
            self.title.expect("Title is required"),
            self.description.expect("Description is required"),
            self.release_date.expect("Release date is required"),
            self.duration_in_minutes.expect("Duration in minutes is required"),
            self.genres.expect("Genres are required"),
            self.language.expect("Language is required"),
        )
    }
}

impl MovieBuilder for MovieBuilderImpl {
    fn set_duration_in_minutes(&mut self, duration_in_minutes: u32) -> &mut Self {
        self.duration_in_minutes = Some(duration_in_minutes);
        self
    }
}