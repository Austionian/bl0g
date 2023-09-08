use serde::de::DeserializeOwned;

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct Project {
    title: String,
    description: String,
    href: String,
    display_link: String,
    image: String,
}

#[derive(Debug)]
pub enum ProjectError {
    ParseError(serde_yaml::Error),
}

impl From<serde_yaml::Error> for ProjectError {
    fn from(value: serde_yaml::Error) -> Self {
        Self::ParseError(value)
    }
}

impl Project {
    pub fn from_file(file: String) -> Result<Self, ProjectError> {
        Ok(deserialize_project::<Self>(&file)?)
    }
}

pub fn deserialize_project<T: DeserializeOwned>(file_string: &str) -> Result<T, ProjectError> {
    Ok(serde_yaml::from_str(file_string.as_ref())?)
}
