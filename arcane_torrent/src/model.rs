
pub enum MediaType {
    Movie,
    TV,
}

pub enum Resolution {
    SD,
    HD,
    FHD,
    UHD,
}

pub enum Language {
    English,
    Spanish,
    French,
    German,
    Italian,
    Portuguese,
    Russian,
    Japanese,
    Korean,
    Chinese,
}

#[derive(Debug)]
pub struct Magnet(pub String);

#[derive(Debug)]
pub enum Media {
    Movie {
        title: String,
        size: String,
        description: String,
        magnet: Magnet,
        seeders: i32,
        leechers: i32,
    },
    TV {
        title: String,
        episode: String,
        season: String,
        size: String,
        description: String,
        magnet: Magnet,
        seeders: i32,
        leechers: i32,
    },
}
