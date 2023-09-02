use axum::http::HeaderMap;

/// A function for  getting either a complete hypermedia response
/// with header, footer, layout etc, or a fragment of just the
/// requested content.
///
/// Allows for returning templates in an SPA
/// or MPA method.
///
/// # Examples
///
/// ```
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
/// ```
pub fn get_template(headers: &HeaderMap, template_name: &str) -> String {
    if headers.get("HX-Request").is_some_and(|v| v == "true") {
        format!("{template_name}.html")
    } else {
        format!("{template_name}_full.html")
    }
}
