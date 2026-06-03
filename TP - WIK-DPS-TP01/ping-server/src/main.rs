use std::env;
use std::net::TcpListener;
use std::io::{BufRead, BufReader, Write};

fn main() {
    let port = env::var("PING_LISTEN_PORT").unwrap_or_else(|_| "8080".to_string());
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).expect("Impossible de démarrer le serveur");
    
    println!("Serveur démarré sur le port {}", port);

    for stream in listener.incoming() {
        match stream {
            Ok(mut s) => {
                let reader = BufReader::new(s.try_clone().unwrap());
                let mut lines = reader.lines();

                let request_line = lines.next().unwrap().unwrap();
                let mut parts = request_line.split_whitespace();
                let method = parts.next().unwrap_or("").to_string();
                let path = parts.next().unwrap_or("").to_string();
                println!("{} {}", method, path);
                
                let mut headers: Vec<(String, String)> = Vec::new();
                for line in lines {
                    let line = line.unwrap();
                    if line.is_empty() { break; }
                    if let Some((key, value)) = line.split_once(": ") {
                        headers.push((key.to_string(), value.to_string()));
                    }
                }

                if method == "GET" && path == "/ping" {
                    let json_pairs: Vec<String> = headers
                        .iter()
                        .map(|(k, v)| format!("  \"{}\": \"{}\"", k, v))
                        .collect();
                    let body = format!("{{\n{}\n}}", json_pairs.join(",\n"));

                    let response = format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
                        body.len(), body
                    );
                    s.write_all(response.as_bytes()).unwrap();
                } else {
                    s.write_all(b"HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\n\r\n").unwrap();
                }
            },
            Err(e) => eprintln!("Erreur : {}", e),
        }
    }
}