use std::io::{self, BufRead};
use regex::Regex;
use std::str::FromStr;

fn main() {
    let diff_re = Regex::new(r"^diff --git a/(.*) b/(.*)$").unwrap();
    let todo_re = Regex::new(r"\+.*# TODO(:)? (.*)$").unwrap();
    let lineno_re = Regex::new(r"^@@ -.* \+([0-9]+),[0-9]+ @@").unwrap();

    let stdin = io::stdin();
    let mut current_file = String::new();
    let mut data: Vec<(String, String)> = Vec::new();
    let mut line_counter: u32 = 0;
    let mut line_offset: u32 = 0;

    for line in stdin.lock().lines() {
        let l = match line {
            Ok(line) => line,
            Err(e) => {
                eprintln!("Error reading line: {}", e);
                continue; // Skip this line and continue with the next
            }
        };

        if l.starts_with("diff") {
            if let Some(caps) = diff_re.captures(&l) {
                current_file = caps[2].to_string();
            }
        } else if let Some(caps) = lineno_re.captures(&l) {
            line_counter = 0;
            line_offset = u32::from_str(&caps[1]).unwrap();
        } else {
            if let Some(caps) = todo_re.captures(&l) {
                data.push((format!("{}:{}", current_file.clone(), line_offset+line_counter), caps[2].to_string()));
            };
            if !l.starts_with("-") {
                line_counter += 1;
            }
            
        }
    }

    let max_length = data.iter()
    .map(|s| s.0.len())
    .max()
    .unwrap_or(0) + 1;

    for (file, todo) in &data {
        println!("./{:<max_length$} TODO: {}", file, todo);
    }
}
