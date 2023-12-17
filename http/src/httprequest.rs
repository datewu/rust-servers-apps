use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Resource {
    Path(String),
}

#[derive(Debug)]
pub struct HTTPRequest {
    pub method: Method,
    pub version: Version,
    pub resource: Resource,
    pub headers: HashMap<String, String>,
    pub msg_body: String,
}

fn process_req_line(s: &str) -> (Method, Resource, Version) {
    let mut words = s.split_whitespace();
    let method = words.next().unwrap();
    let resource = words.next().unwrap();
    let version = words.next().unwrap();

    (
        method.into(),
        Resource::Path(resource.to_string()),
        version.into(),
    )
}

fn process_header_line(s: &str) -> (String, String) {
    let mut headers = s.split(":");
    let key = headers.next().unwrap_or("");
    let value = headers.next().unwrap_or("");

    (key.to_string(), value.to_string())
}
impl From<String> for HTTPRequest {
    fn from(req: String) -> Self {
        let mut parse_method = Method::Uninitialized;
        let mut parse_version = Version::V1_1;
        let mut parse_resource = Resource::Path("".into());
        let mut parse_headers = HashMap::new();
        let mut parsed_msg_body = "";

        for line in req.lines() {
            if line.contains("HTTP") {
                let (method, resource, version) = process_req_line(line);
                parse_method = method;
                parse_resource = resource;
                parse_version = version;
            } else if line.contains(":") {
                let (key, value) = process_header_line(line);
                parse_headers.insert(key, value);
            } else if line.len() == 0 {
                // todo
            } else {
                parsed_msg_body = line;
            }
        }
        Self {
            method: parse_method,
            version: parse_version,
            resource: parse_resource,
            headers: parse_headers,
            msg_body: parsed_msg_body.to_string(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Method {
    Get,
    Post,
    Uninitialized,
}

impl From<&str> for Method {
    fn from(s: &str) -> Self {
        match s {
            "GET" => Self::Get,
            "POST" => Self::Post,
            _ => Self::Uninitialized,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Version {
    V1_1,
    V2_0,
    Uninitialized,
}

impl From<&str> for Version {
    fn from(s: &str) -> Self {
        match s {
            "HTTP/1.1" => Self::V1_1,
            _ => Self::Uninitialized,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_method_into() {
        let m: Method = "GET".into();
        assert_eq!(m, Method::Get);
    }

    #[test]
    fn test_version_into() {
        let m: Version = "HTTP/1.1".into();
        assert_eq!(m, Version::V1_1);
    }

    #[test]
    fn test_read_http() {
        let s = String::from("GET /greeting HTTP/1.1\r\nHOST:localhost:3000\r\nUser-Agent:curl/7.4.1\r\nAccep:*/*\r\n");
        let mut header_expected = HashMap::new();
        header_expected.insert("HOST".to_string(), "localhost".to_string());
        header_expected.insert("Accep".into(), "*/*".into());
        header_expected.insert("User-Agent".into(), "curl/7.4.1".into());

        let req: HTTPRequest = s.into();
        assert_eq!(Method::Get, req.method);
        assert_eq!(Version::V1_1, req.version);
        assert_eq!(Resource::Path("/greeting".to_string()), req.resource);
        assert_eq!(header_expected, req.headers);
    }
}
