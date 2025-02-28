use crate::helpers;
use chrono::{DateTime, Utc};
use comrak::{ComrakOptions, markdown_to_html};
use serde::de::DeserializeOwned;
use std::fmt::Display;

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct FrontMatter {
    pub id: uuid::Uuid,
    pub title: String,
    pub date: DateTime<Utc>,
    description: String,
    pub draft: Option<bool>,
}

#[derive(Debug)]
pub enum FrontmatterError {
    ParseError(serde_yaml::Error),
    MissingFrontMatter,
}

impl From<serde_yaml::Error> for FrontmatterError {
    fn from(value: serde_yaml::Error) -> Self {
        Self::ParseError(value)
    }
}

impl FrontMatter {
    pub fn new(title: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            title,
            date: chrono::Utc::now(),
            description: String::default(),
            draft: Some(true),
        }
    }

    pub fn from_file(file: String) -> Result<Self, FrontmatterError> {
        Ok(deserialize_frontmatter::<Self>(&file)?.0)
    }

    /// Reads the source md file and gets its post content and converts that
    /// to html.
    fn get_content(&self) -> Result<String, FrontmatterError> {
        let md =
            helpers::read_post_to_string(&self.title).unwrap_or("Unable to load post.".to_string());
        let content = deserialize_frontmatter::<Self>(&md)?.1;
        Ok(markdown_to_html(&content, &ComrakOptions::default()))
    }

    pub fn to_file(&self) -> String {
        format!(
            r#"---
id: {}
title: {}
date: {}
description: {}
draft: {}
---"#,
            self.id,
            self.title,
            self.date,
            self.description,
            self.draft.unwrap_or(true)
        )
    }
}

/// Uses the front matter to convert the post into an
/// rss entry.
impl Display for FrontMatter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let url = format!("https://r00ks.io/bl0g/{}", self.title);
        let readable_title = self.title.replace('_', " ");
        let content = self
            .get_content()
            .unwrap_or("Unable to load post".to_string());
        write!(
            f,
            r#"
            <entry>
                <title>{}</title>
                <description>{}</description>
                <link rel="alternate" href="{}" type="text/html" title="{}"/>
                <published>{}</published>
                <id>{}</id>
                <content type="html" xml:base="https://r00ks.io/bl0g/{}">{}</content>
                <author>
                <name>Austin Rooks</name>
                </author>
            </entry>"#,
            readable_title, self.description, url, self.title, self.date, url, self.title, content
        )
    }
}

pub fn deserialize_frontmatter<T: DeserializeOwned>(
    file_string: &str,
) -> Result<(T, String), FrontmatterError> {
    if !file_string.starts_with("---") {
        return Err(FrontmatterError::MissingFrontMatter);
    }

    let split_data = file_string
        .split("---")
        .map(Into::into)
        .collect::<Vec<String>>();

    let frontmatter = match split_data.get(1) {
        Some(f) => Ok(f),
        None => Err(FrontmatterError::MissingFrontMatter),
    }?;

    let content = match split_data.get(2) {
        Some(s) => s.clone(),
        None => String::new(),
    };

    Ok((serde_yaml::from_str(frontmatter.as_ref())?, content))
}
