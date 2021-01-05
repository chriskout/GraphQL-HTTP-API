use http_req::request;
use std::net::TcpStream;
use http_req::response::Headers;
use http_req::request::Request;
use http_req::request::Method;
use http_req::tls::Conn;
use http_req::{request::RequestBuilder, tls, uri::Uri, response::StatusCode};

//Unfortunately I do not know if any of these rquests are actually doing anything.
//I just know that the status code is a success.

//Sanity Check
pub fn basicRequest() {
    let mut writer = Vec::new(); //container for body of a response
    let uri = "http://localhost:8080/";
    let res = request::get(uri, &mut writer).unwrap();

    println!("Status: {} {}", res.status_code(), res.reason());
}

pub fn loadSchema(filepath : &str) -> http_req::response::Response {
    let mut writer = Vec::new();
    let uri : Uri = "http://localhost:8080/admin/schema".parse().unwrap();
    let mut headers = Headers::new();
    headers.insert("Accept-Charset", "utf-8");
    headers.insert("Accept-Language", "en-US");

    let response = Request::new(&uri).headers(headers).method(Method::POST).body(b"placeholder").send(&mut writer).unwrap();
    return response
}

//Below funciton inspired by first example @ https://docs.rs/http_req/0.7.2/http_req/request/struct.RequestBuilder.html

pub fn startConnection() -> Conn<TcpStream> {
    let graphBase = "http://localhost:8080/graphql";
    let addr: Uri = graphBase.parse().unwrap();
    let stream = TcpStream::connect((addr.host().unwrap(), addr.corr_port())).unwrap();
    let mut stream = tls::Config::default()
            .connect(addr.host().unwrap_or(""), stream)
                .unwrap();
    return stream

}
//This is what I would try to get setup first
/*
pub fn makeQuery(str s) {
    let connection = startConnection();
    let body = readFile(s);  //add this
    let mut writer = Vec::new();
    let response = RequestBuilder()::new(&addr)
        .method(METHOD:POST)
        .header("Content-Length", &body.len())
        .header("Content-Type", "application/dql")
        .body(body)
        .send(stream, writer)
        .unwrp()
}
*/
