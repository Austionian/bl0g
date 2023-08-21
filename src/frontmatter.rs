use serde::de::DeserializeOwned;

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
