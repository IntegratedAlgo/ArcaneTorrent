use time::{Date, format_description::well_known::Iso8601};

use arctor_model::{Language, Person, Movie, MovieBuilder, MovieBuilderImpl};
use async_trait::async_trait;
use tvdb_client::{apis::{configuration::Configuration, login_api, search_api::get_search_results}, models::LoginPostRequest};

use super::MediaExplorer;

pub trait MovieExplorer : MediaExplorer {
    fn with_filter_for_duration(&mut self, duration_in_minutes: fn(u32) -> bool) -> &mut Self;
}

pub struct MovieExplorerImpl {
    title: Option<String>,
    year: Option<u32>,
    language: Option<Language>,
    director: Option<Person>,
    genre: Option<Vec<String>>,
    duration_in_minutes: Option<fn(u32) -> bool>,
    configuration: Configuration,
}

impl MovieExplorerImpl {
    pub fn new() -> Self {
        Self {
            title: None,
            year: None,
            language: None,
            director: None,
            genre: None,
            duration_in_minutes: None,
            configuration: Configuration::new(),
        }
    }

    pub async fn authenticate(&mut self, api_key: &str) -> &mut Self {
        let bearer_token = match login_api::login_post(&self.configuration, LoginPostRequest {
            apikey: api_key.to_string(),
            pin: None
        }).await {
            Ok(token) => token,
            Err(e) => panic!("Unable to authenticate: {}", e),
        };

        self.configuration.bearer_access_token = Some(bearer_token.data.expect("Unable to access data").token.expect("Unable to read token"));

        self
    } 
}

#[async_trait]
impl MediaExplorer for MovieExplorerImpl {
    type OutputType = Vec<Movie>;

  
    fn query(&mut self, title: &str) -> &mut Self {
        self.title = Some(title.to_string());
        self
    }

    fn with_year(&mut self, year: u32) -> &mut Self {
        self.year = Some(year);
        self
    }

    fn with_language(&mut self, language: &Language) -> &mut Self {
        self.language = Some(language.to_owned());
        self
    }

    fn with_director(&mut self, director: &Person) -> &mut Self {
        self.director = Some(director.to_owned());
        self
    }

    fn with_genres(&mut self, genres: Vec<String>) -> &mut Self {
        self.genre = Some(genres);
        self
    }

    async fn execute(&self) -> Self::OutputType {
        if(self.configuration.bearer_access_token.is_none()) {
            panic!("Unable to execute query without authentication");
        }

        let name: String;
        let lang : String;
        
        let year : Option<f32> = match self.year {
            Some(y) => Some(y as f32),
            None => None,
        };

        let title = match &self.title {
            Some(t) => Some(t.as_str()),
            None => None,
        };

        let director = match &self.director {
            Some(d) => match &d.name {
                Some(n) => Some({
                    name = n.to_string();
                    name.as_str()
                }),
                None => None,
            }
            None => None,
        };

        let language = match &self.language {
            Some(l) => Some({
                lang = l.to_string();
                lang.as_str()
            }),
            None => None,
        };
            

        let results = get_search_results(&self.configuration, title, None, None, year, None, None, director, language, None, None, None, None, None).await.unwrap();

        let mut movies : Vec<Movie> = Vec::new();

        for movie in results.data.expect("Unable to access data") {

            let parsed_movie = Movie {
                tvdb_id: movie.tvdb_id.expect("Unable to access tvdb id").parse().unwrap(),
                title: movie.name,
                description: movie.overview,
                duration_in_minutes: None,
                genres: movie.genres,
                language: Language::try_from(movie.primary_language)
            };
                
            movies.push(parsed_movie);
        }

        movies
    }
}

impl MovieExplorer for MovieExplorerImpl {
    fn with_filter_for_duration(&mut self, duration_in_minutes: fn(u32) -> bool) -> &mut Self {
        self.duration_in_minutes = Some(duration_in_minutes);
        self
    }
}