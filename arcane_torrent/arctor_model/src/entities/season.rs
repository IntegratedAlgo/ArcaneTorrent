
#[derive(Debug, Clone)]

pub struct Season {
    pub tvdb_id: u32,
    pub season_number: u8,

    pub series_tvdb_id: u32,
    pub episode_tvdb_ids: Vec<u32>
}

