use super::super::error::SabaniError;
use std::process::Command;

pub fn lookup_host(url: &String) -> Result<Vec<super::SabaniIpAddr>, SabaniError> {
  let output = Command::new("nslookup").args([url]).output();
  match output {
    Ok(output) => {
      let output_str = output.stdout;
      let output_str = std::str::from_utf8(&output_str).unwrap();
      let mut v = Vec::new();
      let mut ok = false;
      for line in output_str.lines() {
        if line.contains("Non-authoritative answer:") {
          ok = true;
        }
        if ok && line.contains("Address:") {
          let mut l = line.split(" ");
          println!("{l:?}");
          let addr = l.nth(1).unwrap();
          let addr = super::SabaniIpAddr::new(addr.to_string());
          if let Ok(addr) = addr {
            // エラーは一旦無視する
            v.push(addr)
          }
        }
      }
      Ok(v)
    }
    Err(_) => Err(SabaniError::Network("Failed to run nslookup".to_string())),
  }
}
