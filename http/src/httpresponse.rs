use std::collections::HashMap;
use std::io::{Result, Write};
#[derive(Debug, PartialEq, Clone)]
pub struct HttpResponse<'a> {
    version: &'a str,
    status_code: &'a str,
    status_text: &'a str,
    headers: Option<HashMap<&'a str, &'a str>>,
    body: Option<String>,
}

impl<'a> From<HttpResponse<'a>> for String {
    fn from(res: HttpResponse) -> Self {
        let r = res.clone();
        format!(
            "{} {} {}\r\n{}Content-Length:{}\r\n\r\n{}",
            r.version(),
            r.status_code(),
            r.status_text(),
            r.headers(),
            res.body.unwrap().len(),
            r.body()
        )
    }
}
impl<'a> Default for HttpResponse<'a> {
    fn default() -> Self {
        Self {
            version: "HTTP/1.1",
            status_code: "200",
            status_text: "OK",
            headers: None,
            body: None,
        }
    }
}

impl<'a> HttpResponse<'a> {
    pub fn new(
        status_code: &'a str,
        headers: Option<HashMap<&'a str, &'a str>>,
        body: Option<String>,
    ) -> Self {
        let mut response = HttpResponse::default();
        if status_code != "200" {
            response.status_code = status_code;
        };
        response.headers = match &headers {
            Some(_) => headers,
            None => {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            }
        };
        response.status_text = match response.status_code {
            "200" => "OK",
            "400" => "Bad Request",
            "500" => "Internal Server Error",
            _ => "Not Found",
        };
        response.body = body;
        response
    }

    pub fn send_response(&self, write_stream: &mut impl Write) -> Result<()> {
        let res = self.clone();
        let response_string = String::from(res);
        write!(write_stream, "{}", response_string)?;
        Ok(())
    }
}

impl<'a> HttpResponse<'a> {
    fn version(&self) -> &str {
        self.version
    }
    fn status_code(&self) -> &str {
        self.status_code
    }
    fn status_text(&self) -> &str {
        self.status_text
    }
    fn headers(&self) -> String {
        let m: HashMap<&str, &str> = self.headers.clone().unwrap();
        let mut accu: String = "".to_string();
        for (k, v) in m.iter() {
            accu = format!("{}{}:{}\r\n", accu, k, v);
        }
        accu
    }
    pub fn body(&self) -> &str {
        match &self.body {
            Some(b) => b.as_str(),
            None => "",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_response_200() {
        let response_actual = HttpResponse::new("200", None, Some("abc".to_string()));
        let response_expected = HttpResponse {
            version: "HTTP/1.1",
            status_code: "200",
            status_text: "OK",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some("abc".to_string()),
        };
        assert_eq!(response_actual, response_expected);
    }

    #[test]
    fn test_response_404() {
        let response_actual = HttpResponse::new("404", None, Some("abc".to_string()));
        let response_expected = HttpResponse {
            version: "HTTP/1.1",
            status_code: "404",
            status_text: "Not Found",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some("abc".to_string()),
        };
        assert_eq!(response_actual, response_expected);
    }

    #[test]
    fn test_into_string() {
        let respones = HttpResponse {
            version: "HTTP/1.1",
            status_code: "404",
            status_text: "Not Found",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some("abc".to_string()),
        };
        let http_string: String = respones.into();
        let http_actual =
            "HTTP/1.1 404 Not Found\r\nContent-Type:text/html\r\nContent-Length:3\r\n\r\nabc";
        assert_eq!(http_string, http_actual);
    }
}
