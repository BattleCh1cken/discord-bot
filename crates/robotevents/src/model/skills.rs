use crate::IdInfo;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Skill {
    pub id: i32,
    pub event: IdInfo,
    pub team: IdInfo,
    pub _type: String,
    pub season: IdInfo,
    pub division: IdInfo,
    pub rank: i32,
    pub score: i32,
    pub attempts: i32,
}
