use itertools::Itertools;
use serde_xml_rs::from_str;

use crate::blog::Blog;

use self::{atom::Feed, rss::Rss, traits::ResultToBlog};

pub mod atom;
pub mod rss;
mod traits;

/// Turns an XML feed into a `Blog` if possible.
pub fn parse_web_feed(xml: &str) -> Result<Blog, String> {
  let possible_roots = vec![
    from_str::<Rss>(xml).into_blog(),
    from_str::<Feed>(xml).into_blog(),
  ];

  let (roots, errors): (Vec<_>, Vec<_>) = possible_roots.into_iter().partition_result();

  roots
    .first()
    .cloned()
    .ok_or_else(|| format!("{:?}", errors.iter().unique().collect::<Vec<_>>()))
}

#[cfg(test)]
mod tests {
  use super::parse_web_feed;
  use crate::blog::{Blog, Post};
  use chrono::{DateTime, FixedOffset};

  fn read_file(dir_name: &str, file_name: &str) -> String {
    use std::fs;
    use std::path::PathBuf;
    let mut file_path = PathBuf::from("test-data");
    file_path.push(dir_name);
    file_path.push(file_name);

    fs::read_to_string(file_path).expect("Cannot read feed ")
  }

  fn read_rss(file_name: &str) -> String {
    read_file("rss-feeds", file_name)
  }

  fn read_atom(file_name: &str) -> String {
    read_file("atom-feeds", file_name)
  }

  fn post_date(value: &str) -> DateTime<FixedOffset> {
    value
      .parse::<DateTime<FixedOffset>>()
      .expect(&format!("Invalid date {}", value))
  }

  #[test]
  fn test_parse_rss_data() {
    let content = read_rss("rss.xml");
    let blog = parse_web_feed(&content).expect("Parsed content");
    println!("{:?}", blog);
  }

  #[test]
  fn test_parse_brief_single_entry_atom() {
    let content = read_atom("brief-single-entry.xml");
    let blog = parse_web_feed(&content).expect("Parsed content");

    assert_eq!(
      blog,
      Blog {
        title: "Example Feed".into(),
        last_build_date: post_date("2003-12-13T18:30:02+00:00"),
        posts: vec![Post {
          title: "Atom-Powered Robots Run Amok".into(),
          link: "http://example.org/2003/12/13/atom03".into(),
          description: Some("Some text.".into()),
          last_build_date: post_date("2003-12-13T18:30:02+00:00"),
        }],
      }
    );
  }
  #[test]
  fn test_parse_complex_single_entry_atom() {
    let content = read_atom("complex-single-entry.xml");
    let blog = parse_web_feed(&content).expect("Parsed content");

    // The chosen link is the .../2005/04/02/atom because it is the first in the list
    assert_eq!(
      blog,
      Blog {
        title: "dive into mark".into(),
        last_build_date: post_date("2005-07-31T12:29:29+00:00"),
        posts: vec![Post {
          title: "Atom draft-07 snapshot".into(),
          link: "http://example.org/2005/04/02/atom".into(),
          description: None,
          last_build_date: post_date("2005-07-31T12:29:29+00:00"),
        }],
      });
  }

  #[test]
  fn test_parse_feed_without_entry() {}

  #[test]
  fn test_parse_feed_with_many_entries() {}

  // Ignored:
  // - multiple feeds in a single document
}
