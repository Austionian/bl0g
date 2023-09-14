use axum::http::HeaderMap;

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
/// let mut output_headers = HeaderMap::new();
/// output_headers.insert("Vary", "HX-Request".parse().unwrap());
///
/// assert_eq!(get_headers_and_template(&headers, "test"), (output_headers, "test.html".to_string());
///
/// let mut value = headers.entry("HX-Request").or_insert("false".parse().unwrap());
/// *value = "false".parse().unwrap();
/// assert_eq!(get_headers_and_template(&headers, "test"), (HeaderMap::default(), "test_full.html".to_string()));
pub fn get_headers_and_template(headers: &HeaderMap, template_name: &str) -> (HeaderMap, String) {
    if headers.get("HX-Request").is_some_and(|v| v == "true") {
        let mut headers = HeaderMap::new();
        headers.insert("Vary", "HX-Request".parse().unwrap());
        (headers, format!("partials/{template_name}.html"))
    } else {
        (HeaderMap::default(), format!("{template_name}.html"))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use axum::http::HeaderMap;

    #[test]
    fn get_fragment() {
        let mut headers = HeaderMap::new();
        headers.insert("HX-Request", "true".parse().unwrap());

        let mut output_headers = HeaderMap::new();
        output_headers.insert("Vary", "HX-Request".parse().unwrap());

        assert_eq!(
            get_headers_and_template(&headers, "test"),
            (output_headers, "partials/test.html".to_string())
        );
    }

    #[test]
    fn get_full() {
        let mut headers = HeaderMap::new();
        headers.insert("HX-Request", "false".parse().unwrap());

        assert_eq!(
            get_headers_and_template(&headers, "test"),
            (HeaderMap::default(), "test.html".to_string())
        );
    }
}
