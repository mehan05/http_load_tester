use std::{io::{self, Write}, thread, time::Duration};

use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

pub fn header_spliter(h: String) -> (String, String){
        if let Some((key, value)) = h.split_once(':') {
            (key.trim().to_string(), value.trim().to_string())
        } else {
            eprintln!("Invalid header format: {}", h);
            (String::new(), String::new())
        }
}


pub fn header_mapper(h:&[String])->HeaderMap{
    let mut headermap = HeaderMap::new();
    
    for i in h{
        let (headername,headervalue) = header_spliter(i.clone());

        if let (Ok(key),Ok(val)) = (HeaderName::from_bytes(headername.as_bytes()), HeaderValue::from_bytes(headervalue.as_bytes())){
            headermap.insert(key,val);
        }
        else{
            eprintln!("Invalid header format: {}", i);
        }
    }

    headermap
}

