use std::collections::HashMap;

use http::httprequest::{HttpRequest, Method, Resource, Version};

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
    let s: String = String::from(
        "GET /greeting HTTP/1.1\r\nHost: localhost:3000\r\nUser-Agent: curl/7.64.1\r\nAccept: */*\r\n\r\n",
    );
    let mut headers_expected = HashMap::new();
    headers_expected.insert("Host".into(), " localhost".into());
    headers_expected.insert("Accept".into(), " */*".into());
    headers_expected.insert("User-Agent".into(), " curl/7.64.1".into());
    let req: HttpRequest = s.into();
    assert_eq!(Method::Get, req.method);
    assert_eq!(Version::V1_1, req.version);
    assert_eq!(Resource::Path("/greeting".to_string()), req.resource);
    assert_eq!(headers_expected, req.headers);
}
