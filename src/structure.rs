use chrono::{DateTime, Local};

pub struct LearningList {
    pub id: i32,
    pub url: String,
    pub title: String,
    pub memo: String,
    pub date: DateTime<Local>,
}
