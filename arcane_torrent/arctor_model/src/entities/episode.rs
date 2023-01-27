use time::Date;

pub struct Episode {
    pub tvdb_id: u32,
    pub title: String,
    pub description: String,
    pub release_date: Date,


    pub season_tvdb_id: u32,
}