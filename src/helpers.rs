use axum::http::HeaderMap;

pub fn get_template(headers: HeaderMap, template_name: &str) -> String {
    if headers.get("HX-Request").is_some_and(|v| v == "true") {
        format!("{template_name}.html")
    } else {
        format!("{template_name}_full.html")
    }
}
