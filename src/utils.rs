use url::Url;

pub fn validate_url(url_str: &str) -> bool {
    match Url::parse(url_str) {
        Ok(url) => url.scheme() == "http" || url.scheme() == "https",
        Err(_) => false,
    }
}
