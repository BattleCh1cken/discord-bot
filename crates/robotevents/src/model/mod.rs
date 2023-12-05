use serde::Deserialize;
pub mod skills;

#[derive(Debug, Deserialize)]
pub struct RobotEventsResponse<T> {
    pub meta: Meta,
    pub data: Vec<T>,
}

#[derive(Debug, Deserialize)]
pub struct Meta {
    current_page: i32,
    first_page_url: String,
    from: i32,
    last_page: i32,
    last_page_url: String,
    next_page_url: Option<String>,
    path: String,
    per_page: i32,
    to: i32,
    total: i32,
}

#[derive(Debug, Deserialize)]
pub struct Team {
    pub id: i32,
    pub number: String,
    pub team_name: String,
    pub robot_name: String,
    pub organization: String,
    pub location: Location,
    pub registered: bool,
    pub program: IdInfo,
    pub grade: String,
}

#[derive(Debug, Deserialize)]
pub struct Location {
    pub venue: Option<String>,
    pub address_1: String,
    pub address_2: Option<String>,
    pub city: String,
    pub region: String,
    pub postcode: String,
    pub country: String,
    pub coordinates: Coordinates,
}

#[derive(Debug, Deserialize)]
pub struct Coordinates {
    pub lat: f32,
    pub lon: f32,
}

#[derive(Debug, Deserialize)]
pub struct Season {
    id: i32,
    pub program: IdInfo,
    pub start: String,
    pub years_start: i32,
    pub years_end: i32,
}

#[derive(Debug, Deserialize)]
pub struct IdInfo {
    id: i32,
    pub name: String,
    pub code: String,
}
