use arctor_model::{Person, Language};
use async_trait::async_trait;

mod movie_explorer;

#[async_trait]
pub trait MediaExplorer {
    type OutputType;

    fn query(&mut self, title: &str) -> &mut Self;

    fn with_year(&mut self, year: u32) -> &mut Self;

    fn with_language(&mut self, language: &Language) -> &mut Self;

    fn with_director(&mut self, director: &Person) -> &mut Self;

    fn with_genres(&mut self, genre: Vec<String>) -> &mut Self;

    async fn execute(&self) -> Self::OutputType;
}

pub use movie_explorer::{MovieExplorerImpl, MovieExplorer};