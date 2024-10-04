extern crate time;
extern crate web;
use std::fs::read_to_string;

use web::PageOps;

/* this is an example of a simple Rust CGI script to
   show content of files
   */
struct Page {
    content: String
}

fn main() {
    let page = Page { content:
     if let Ok(current_path) = std::env::var(String::from("PATH_TRANSLATED")) {
        format!{r#"<!doctype html>
<html>
  <head>
    <title>Test content from {current_path}</title>
  </head>
  <body>
    <h1>${{name}}  {current_path}</h1>
    <pre>{}</pre>
  </body>
</html>"#, if let Ok(content) = read_to_string(&current_path) {
                content
            } else {
                format!{"Can't read {current_path}"}.to_string()
            }
        }
     } else {
        String::from(r#"<!doctype html>
<html>
  <head>
    <title>Test content from</title>
  </head>
  <body>
    <p>No ${name}.</p>
  </body>
</html>"#)
    }
    };
    page.show()
}

impl web::PageOps for Page  {
    fn main_load(&self) -> Result<String, String> {
        Ok(self.content.clone())
    }
    
    fn name(&self) -> String {
        "Show".to_string()
    }
}