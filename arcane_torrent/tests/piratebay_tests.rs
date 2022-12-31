#[cfg(test)]
mod tests {
    use arcane_torrent::sites::{*, piratebay::PirateBayExplorer};

    #[tokio::test]
    async fn get_magnet_uk_pirate_bay() {
        let explorer : PirateBayExplorer = PirateBayExplorer::new("https://ukpiratebay.org/".to_owned()).await;
    
        let result = explorer.search(Query {
            title: "The Mandalorian".to_string(),
            year: "2019".to_string(),
            season: 1,
            episode: 1,
            resolution: Resolution::FHD,
            language: "English".to_string(),
            media_type: MediaType::TV,
        }).await;
    }
}
