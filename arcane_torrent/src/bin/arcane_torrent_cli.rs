use arcane_torrent::sites::{piratebay::PirateBayExplorer, Resolution, Query, MediaType, Explorer, Magnet, Media};
use thirtyfour::prelude::*;
use rand::prelude::*;
use stopwatch::{Stopwatch};
use tokio::join;

async fn search(explorer: &PirateBayExplorer, title: String) -> Vec<Media> {
    explorer.search(Query {
        title: "Nanny".to_string(),
        year: "2019".to_string(),
        season: 1,
        episode: 1,
        resolution: Resolution::FHD,
        language: "English".to_string(),
        media_type: MediaType::TV,
    }).await.unwrap()
}
fn get_random_movie() -> &'static str {
    // Create a list of movie names
    let movies = vec![
        "The Shawshank Redemption",
        "The Godfather",
        "The Godfather: Part II",
        "The Dark Knight",
        "12 Angry Men",
        "Schindler's List",
        "The Lord of the Rings: The Return of the King",
        "Pulp Fiction",
        "The Good, the Bad and the Ugly",
        "The Lord of the Rings: The Fellowship of the Ring",
        "Fight Club",
        "Forrest Gump",
        "Inception",
        "The Matrix",
        "The Silence of the Lambs",
        "One Flew Over the Cuckoo's Nest",
        "Goodfellas",
        "The Green Mile",
        "The Prestige",
        "The Departed",
        "The Lion King",
    ];

    // Generate a random number between 0 and the number of movies
    let index = rng.gen_range(0..movies.len());

    // Return the random movie name
    &movies[index]
}

#[tokio::main]
async fn main() {
    // Create a vector to hold the results
    let mut results = Vec::new();

    // Create 10 tasks and run them concurrently
    for i in 0..10 {
        let explorer : PirateBayExplorer = PirateBayExplorer::new("https://ukpiratebay.org/".to_owned()).await;
        results.push(tokio::spawn(search(&explorer, get_random_movie().to_string())));
    }

    // Wait for all tasks to complete and gather the results
    let magnets : Vec<Media> = join!(results);



    for magnet in result.iter() {
        println!("URL: {:?}", magnet);
        println!();
    }
    println!("Query Time: {}ms - Results: {}", sw.elapsed_ms(), result.len());
}