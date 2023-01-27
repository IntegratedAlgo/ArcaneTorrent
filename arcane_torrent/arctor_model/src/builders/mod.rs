use time::Date;

pub trait MediaBuilder {
    type OutputType;

    fn set_tvdb_id(&mut self, tvdb_id: u32) -> &mut Self;
    fn set_title(&mut self, title: String) -> &mut Self;
    fn set_description(&mut self, description: String) -> &mut Self;
    fn set_release_date(&mut self, release_date: Date) -> &mut Self;
    fn set_language(&mut self, language: Language) -> &mut Self;
    fn with_genres(&mut self, genres: Vec<String>) -> &mut Self;

    fn build(self) -> Self::OutputType;
}


mod movie_builder;
mod series_builder;

pub use movie_builder::{MovieBuilder, MovieBuilderImpl};
pub use series_builder::{SeriesBuilder, SeriesBuilderImpl};


use crate::Language;