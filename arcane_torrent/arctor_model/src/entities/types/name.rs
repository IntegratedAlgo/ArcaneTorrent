
#[derive(Debug, Clone)]
pub struct Name {
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String
}

impl Name {
    pub fn to_string(&self) -> String {
        match &self.middle_name {
            Some(middle_name) => format!("{} {} {}", self.first_name, middle_name, self.last_name),
            None => format!("{} {}", self.first_name, self.last_name)
        }
    }
}