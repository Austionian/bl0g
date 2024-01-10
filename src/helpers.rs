use axum::http::HeaderMap;
use std::{fs, io};

/// A function for  getting either a complete hypermedia response
/// with header, footer, layout etc, or a fragment of just the
/// requested content.
///
/// Allows for returning templates in an SPA
/// or MPA method.
///
/// **Requires that _full_ templates end with '_full.html'**
///
/// # Examples
///
/// use axum::http::HeaderMap;
/// use bl0g::helpers::get_template;
///
/// let mut headers = HeaderMap::new();
/// headers.insert("HX-Request", "true".parse().unwrap());
///
/// assert_eq!(get_template(&headers, "test"), "test.html".to_string());
///
/// let mut value = headers.entry("HX-Request").or_insert("false".parse().unwrap());
/// *value = "false".parse().unwrap();
/// assert_eq!(get_template(&headers, "test"), "test_full.html".to_string());
pub fn get_template(headers: &HeaderMap, template_name: &str) -> String {
    if headers.get("HX-Request").is_some_and(|v| v == "true") {
        format!("partials/{template_name}.html")
    } else {
        format!("{template_name}.html")
    }
}

pub fn read_post_to_string(post_name: &str) -> Result<String, io::Error> {
    fs::read_to_string(format!("data/posts/{post_name}.md"))
}

#[cfg(test)]
mod test {
    use super::*;
    use axum::http::HeaderMap;

    #[test]
    fn get_fragment() {
        let mut headers = HeaderMap::new();
        headers.insert("HX-Request", "true".parse().unwrap());

        assert_eq!(
            get_template(&headers, "test"),
            "partials/test.html".to_string()
        );
    }

    #[test]
    fn get_full() {
        let mut headers = HeaderMap::new();
        headers.insert("HX-Request", "false".parse().unwrap());
        assert_eq!(get_template(&headers, "test"), "test.html".to_string());
    }
}
