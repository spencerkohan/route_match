pub trait UrlPathProvider {
    fn path_str(&self) -> &str;
}

pub trait HttpMethodProvider {
    fn method_str(&self) -> &str;
}

impl UrlPathProvider for &str {
    fn path_str(&self) -> &str {
        self
    }
}

impl UrlPathProvider for String {
    fn path_str(&self) -> &str {
        &self
    }
}

impl HttpMethodProvider for &str {
    fn method_str(&self) -> &str {
        self
    }
}

impl HttpMethodProvider for String {
    fn method_str(&self) -> &str {
        &self
    }
}
