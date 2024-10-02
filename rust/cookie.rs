#![feature(duration_constructors)]
use std::time::Duration;
use std::time::SystemTime;
use std::ops::Add;
extern crate time;
extern crate web;

use web::PageOps;
use web::Param;
use web::param;
struct Page {
    content: String
}

fn main() {
println!{"{}", web::html_encode(&"<html>".to_string())};
    let page = Page { content:String::from(r#"<!doctype html>
<html>
  <head>
    <title>Cookie tester</title>
  </head>
  <body>
    <p>${name}.</p>
  </body>
</html>"#)};
    page.show()
}

impl web::PageOps for Page  {
    fn main_load(&self) -> Result<String, String> {
        Ok(self.content.clone())
    }

    fn name(&self) -> String {
        let params = Param::new();
        if let Some(value) = params.cookie("SESSIOM") {
            format!{"cookie set to {value}"}
        } else {
            format!{"cookie will set to 'brrrr' expired on {}", 
              param::http_format_time(SystemTime::now().add(Duration::from_days(1)))}
        }
    }
    
    fn get_extra(&self) -> Option<Vec<(String, String)>> {
        Some([web::new_cookie_header(&"SESSIOM".to_string(),
        &"Brrrrr".to_string(), Some(SystemTime::now().add(Duration::from_days(1))))].to_vec())
    }
}
        