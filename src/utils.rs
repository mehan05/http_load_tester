use std::io::{self, Write};

pub fn header_spliter(h: String) -> (String, String){
        if let Some((key, value)) = h.split_once(':') {
            (key.trim().to_string(), value.trim().to_string())
        } else {
            eprintln!("Invalid header format: {}", h);
            (String::new(), String::new())
        }
}
