use http_req::request;
use std::net::TcpStream;
//Unfortunately I do not know if any of these rquests are actually doing anything.
//I just know that the status code is a success.

const str base = "http://localhost:8080/"

//Sanity Check
pub fn basicRequest() {
        let mut writer = Vec::new(); //container for body of a response
        let res = request::get(base, &mut writer).unwrap();

        println!("Status: {} {}", res.status_code(), res.reason());
}

pub fn loadSchema(str filepath) {
    let mut writer = Vec::new();
    let uri = base + "admin/schema";
    let mut headers = Headers::new();
    headers.insert("Accept-Charset", "utf-8");
    headers.insert("Accept-Language", "en-US");

    let response = Request::new(&uri).headers(headers).method(Method::POST).body().send(writer).unwrap();
    return response
}

pub fn startConnection() {
    let graphBase = base + "graphql"
    let addr: Uri = graphBase.parse().unwrap();
    let mut writer = Vec::new();

    let stream = TcpStream::connect((addr.host().unwrap(), addr.corr_port())).unwrap();
    let mut stream = tls::Config::default()
            .connect(addr.host().unwrap_or(""), stream)
                .unwrap()

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
/*
