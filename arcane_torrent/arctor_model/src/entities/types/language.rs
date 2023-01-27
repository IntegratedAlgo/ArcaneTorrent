#[derive(Debug, Clone)]
pub enum Language {
    German,
    English,
    Spanish,
    French
}

impl Language {
    pub fn to_string(&self) -> String {
        match self {
            Language::German => "GER".to_string(),
            Language::English => "ENG".to_string(),
            Language::Spanish => "ESP".to_string(),
            Language::French => "FRA".to_string()
        }
    }

    pub fn try_from(language: Option<String>) -> Option<Self> {
        match language {
            Some(lang) => match lang.to_uppercase().as_str() {
                "GER" => Some(Language::German),
                "ENG" => Some(Language::English),
                "ESP" => Some(Language::Spanish),
                "FRA" => Some(Language::French),
                _ => None
            },
            None => None
        }
    }
}

impl From<&str> for Language {
    fn from(language: &str) -> Self {
        match language.to_uppercase().as_str() {
            "GER" => Language::German,
            "ENG" => Language::English,
            "ESP" => Language::Spanish,
            "FRA" => Language::French,
            _ => panic!("Language {} is not supported", language)
        }
    }
}