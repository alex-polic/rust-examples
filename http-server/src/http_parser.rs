use std::fmt::Error;

pub(crate) fn read_http1x_request(request: &String) -> HttpRequest{
    let mut method: String = String::new();
    let mut path: String = String::new();
    let mut version: String = String::new();
    let mut headers = Vec::<HttpHeader>::new();

    let mut is_first_line = true;
    let mut are_headers_read = false;
    let lines = request.split("\r\n");
    for line in lines {
        if is_first_line {
            let mut first_line_items = line.split(' ');
            method = first_line_items.next().unwrap().trim().to_string();
            path = first_line_items.next().unwrap().trim().to_string();
            version = first_line_items.next().unwrap().trim().to_string();

            is_first_line = false;
            continue;
        }

        if are_headers_read == false {
            if line.is_empty() {
                are_headers_read = true;
                continue;
            }

            let mut header_items = line.split(':');
            headers.push(HttpHeader { 
                name: header_items.next().unwrap().trim().to_string(), 
                value: header_items.next().unwrap().trim().to_string() 
            })
        }
    }


    HttpRequest { is_valid:method.is_empty() == false , method: HttpMethod::from_str(&method).unwrap(), path, version, headers }
}

#[derive(Debug)]
pub(crate) struct HttpRequest {
    pub(crate) is_valid: bool,

    pub(crate) method: HttpMethod,
    pub(crate) path: String,
    pub(crate) version: String,
    pub(crate) headers: Vec<HttpHeader>,
}

#[derive(Debug)]
pub(crate) struct HttpHeader {
    pub(crate) name: String,
    pub(crate) value: String
}

#[derive(Debug)]
pub(crate) enum HttpMethod {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE
}

impl HttpMethod {
    pub(crate) fn from_str(input: &str) -> Result<HttpMethod, Error> {
        match input.to_ascii_lowercase().as_str() {
            "get" => Ok(HttpMethod::GET),
            "post" => Ok(HttpMethod::POST),
            "put" => Ok(HttpMethod::PUT),
            "patch" => Ok(HttpMethod::PATCH),
            "delete" => Ok(HttpMethod::DELETE),
            _ => Err(Error)
        }
    }
}
