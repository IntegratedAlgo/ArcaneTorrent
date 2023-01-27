#[cfg(test)]
mod tests {
    use arcane_torrent::{explorer::{*, piratebay::PirateBayExplorer}, model::{MediaType, Resolution}};

    #[tokio::test]
    async fn get_magnet_uk_pirate_bay() {
        let explorer : PirateBayExplorer = PirateBayExplorer::new("https://ukpiratebay.org/".to_owned()).await;
    
        let result = explorer.search(Query {
            title: "Sturm der Liebe".to_string(),
            year: "2019".to_string(),
            season: 1,
            episode: 1,
            resolution: Resolution::FHD,
            language: "English".to_string(),
            media_type: MediaType::TV, 
        }, 2).await.unwrap();
        
        println!("Length: {:?}", result.len());
        assert!(result.len() == 200);
        drop(explorer)
    }
}
