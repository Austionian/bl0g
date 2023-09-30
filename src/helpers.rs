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
/// assert_eq!(get_template(&headers, "test"), "test.html".to_string());
///
/// let mut value = headers.entry("HX-Request").or_insert("false".parse().unwrap());
/// *value = "false".parse().unwrap();
/// assert_eq!(get_template(&headers, "test"), "test_full.html".to_string());
fn get_template(headers: &HeaderMap, template_name: &str) -> String {
    if headers.get("HX-Request").is_some_and(|v| v == "true") {
        format!("partials/{template_name}.html")
    } else {
        format!("{template_name}.html")
    }
}

fn get_headers(path: &str) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("HX-PUSH-Url", path.parse().unwrap());

    headers
}

pub fn get_headers_and_template(
    headers: &HeaderMap,
    template_name: &str,
    path: &str,
) -> (HeaderMap, String) {
    (get_headers(path), get_template(headers, template_name))
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

    #[test]
    fn creates_the_headers_correctly() {
        let headers = get_headers("foo");
        let mut test_headers = HeaderMap::new();
        test_headers.insert("HX-PUSH-Url", "foo".parse().unwrap());

        assert_eq!(headers, test_headers);
    }
}
