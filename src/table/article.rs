use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Article {
    pub first_author: Option<String>,
    pub year_published: Option<i32>,
    pub title: Option<String>,
    pub summary: Option<String>,
    pub doi: Option<String>,
    pub citations: Option<i32>,
    pub score: Option<i32>
}

impl <'a> Article {
    pub fn get(&'a self, name: &str) -> Option<String> {
        let a = match name {
            "Doi" => self.doi.clone()?,
            "Title" => self.title.clone()?,
            "First author" => self.first_author.clone()?,
            "Abstract" => self.summary.clone()?,
            "Year published" => self.year_published?.to_string(),
            "Citations" => self.citations?.to_string(),
            "Score" => self.score?.to_string(),
            _ => None?
        };
        Some(a)
    }

    pub fn matches(&self, regex: &regex::Regex) -> bool {
        fn unwrap_int(year: &Option<i32>) -> Option<String> {
            Some(format!("{}", (*year)?))
        }
        let year_published_str = unwrap_int(&self.year_published).unwrap_or_default();
        let score = unwrap_int(&self.score).unwrap_or_default();
        let citations = unwrap_int(&self.citations).unwrap_or_default();
    
        regex.is_match(&self.doi.clone().unwrap_or_default()) |
        regex.is_match(&self.title.clone().unwrap_or_default()) |
        regex.is_match(&self.summary.clone().unwrap_or_default()) |
        regex.is_match(&self.first_author.clone().unwrap_or_default()) |
        regex.is_match(&year_published_str) |
        regex.is_match(&score) |
        regex.is_match(&citations)
    }
}

