use chrono::{DateTime, FixedOffset};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Blog {
  pub title: String,
  pub last_build_date: DateTime<FixedOffset>,
  pub posts: Vec<Post>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Post {
  pub title: String,
  pub link: String,
  pub description: Option<String>,
  pub last_build_date: DateTime<FixedOffset>,
}
