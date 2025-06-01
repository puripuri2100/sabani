use sabani_core::net::HttpClient;

fn main() {
  let client = HttpClient::new();
  let response = client.get("localhost".to_string(), 8000, "/test.html".to_string()).unwrap();
  println!("{response:?}");
}
