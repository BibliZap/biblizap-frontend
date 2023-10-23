use serde::{Deserialize, Serialize};

use crate::table::Filters;

fn unwrap_int(year: &Option<i32>) -> Option<String> {
    Some(format!("{}", (*year)?))
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Article {
    pub first_author: Option<String>,
    pub year_published: Option<i32>,
    pub journal: Option<String>,
    pub title: Option<String>,
    pub summary: Option<String>,
    pub doi: Option<String>,
    pub citations: Option<i32>,
    pub score: Option<i32>
}

impl Article {
    pub fn matches_global(&self, regex: &regex::Regex) -> bool {
        let year_published = unwrap_int(&self.year_published).unwrap_or_default();
        let score = unwrap_int(&self.score).unwrap_or_default();
        let citations = unwrap_int(&self.citations).unwrap_or_default();
    
        regex.is_match(&self.doi.clone().unwrap_or_default()) |
        regex.is_match(&self.title.clone().unwrap_or_default()) |
        regex.is_match(&self.journal.clone().unwrap_or_default()) |
        regex.is_match(&self.summary.clone().unwrap_or_default()) |
        regex.is_match(&self.first_author.clone().unwrap_or_default()) |
        regex.is_match(&year_published) |
        regex.is_match(&score) |
        regex.is_match(&citations)
    }

    pub fn matches(&self, filters: &Filters) -> bool {
        let year_published = unwrap_int(&self.year_published).unwrap_or_default();
        let score = unwrap_int(&self.score).unwrap_or_default();
        let citations = unwrap_int(&self.citations).unwrap_or_default();

        filters.doi.regex.is_match(&self.doi.to_owned().unwrap_or_default()) &
        filters.title.regex.is_match(&self.title.to_owned().unwrap_or_default()) &
        filters.journal.regex.is_match(&self.journal.to_owned().unwrap_or_default()) &
        filters.summary.regex.is_match(&self.summary.to_owned().unwrap_or_default()) &
        filters.first_author.regex.is_match(&self.first_author.to_owned().unwrap_or_default()) &
        filters.year_published.regex.is_match(&year_published) &
        filters.score.regex.is_match(&score) &
        filters.citations.regex.is_match(&citations)
    }
}

