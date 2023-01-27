use time::Date;
use super::types::{Name, Gender};

#[derive(Debug, Clone)]
pub struct Person {
    pub name: Option<Name>,
    pub gender: Option<Gender>,
    pub date_of_birth: Option<Date>
}