use chrono::{DateTime, Utc};
use serde::de::DeserializeOwned;

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
}

impl ToString for FrontMatter {
    fn to_string(&self) -> String {
        let url = format!("https://r00ks.io/bl0g/{}", self.title);
        let readable_title = self.title.replace('_', " ");
        format!(
            r#"<entry>
<title>{}</title>
<description>{}</description>
<link rel="alternate" href="{}" type="text/html" title="{}"/>
<published>{}</published>
<id>{}</id>
<author>
<name>Austin Rooks</name>
</author>
</entry>"#,
            readable_title, self.description, url, self.title, self.date, url
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
