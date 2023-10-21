use regex::Regex;

#[derive(Debug)]
pub struct RegexWrapper {
    pub regex: Regex,
}

impl PartialEq for RegexWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.regex.as_str() == other.regex.as_str()
    }
}

impl Default for RegexWrapper {
    fn default() -> Self {
        Self { regex: Regex::new("").unwrap() }
    }
}

impl From<&str> for RegexWrapper {
    fn from(value: &str) -> Self {
        Self { regex: Regex::new(value).unwrap_or(Regex::new("").unwrap()) }
    }
}

#[derive(Default, PartialEq, Debug)]
pub struct Filters {
    pub first_author: RegexWrapper,
    pub year_published: RegexWrapper,
    pub title: RegexWrapper,
    pub summary: RegexWrapper,
    pub doi: RegexWrapper,
    pub citations: RegexWrapper,
    pub score: RegexWrapper,
}
