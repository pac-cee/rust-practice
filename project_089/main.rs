use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tiny_http::{Server, Request, Response, Method};

fn main() {
    let map = Arc::new(Mutex::new(HashMap::new()));
    let server = Server::http("0.0.0.0:8081").unwrap();
    println!("URL shortener running on http://localhost:8081");
    for request in server.incoming_requests() {
        let map = Arc::clone(&map);
        let url = request.url().to_string();
        match (request.method(), url.as_str()) {
            (&Method::Post, "/shorten") => {
                let mut content = String::new();
                request.as_reader().read_to_string(&mut content).unwrap();
                let key = format!("k{}", map.lock().unwrap().len() + 1);
                map.lock().unwrap().insert(key.clone(), content.clone());
