use seahorse::color;
use serde_json::{self, Value};

//use std::process::exit;

pub fn translate(source : &str, target: &str, words: String) -> String {
    let mut store_word = String::new();
    let source = String::from(source); //source language
    let target = String::from(target); //target language
    let words = words.replace('%', " ");
    let args: Vec<String> = vec![words];

    // generate_url(args, source, target);

    let url = generate_url(args, source, target);
    let mut ping: bool = true;
    let mut error: String = String::new();
    let v = reqwest::blocking::get(&url)
        //.and_then(|resp| resp.text())
        //.and_then(|body| Ok(serde_json::from_str::<Vec<Value>>(&body)))
        .and_then(|resp| resp.text())
        .map(|body| serde_json::from_str::<Vec<Value>>(&body))
        .unwrap_or_else(|_| {
            eprintln!("{}", color::red("Network error..."));
            ping = false;
            error = "Network error".to_string();
            Ok(vec![])
        })
        .unwrap_or_else(|_| {
            eprintln!("{}", color::red("JSON parse error..."));
            ping = false;
            error = "JSON parse Error".to_string();
            vec![]
        });
    if ping {
        match v.first() {
            Some(item) => {
                let result = item
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|s| s[0].as_str().unwrap())
                    .collect::<Vec<&str>>()
                    .join(" ");

                store_word = result;
            }

            None => eprintln!("{}", color::red("Error...")),
        }

        store_word
    } else {
        error
    }
}

fn generate_url(v: Vec<String>, source: String, target: String) -> String {
    let base_url = "https://translate.googleapis.com/translate_a/single";
    let q = v.join(" ");
    format!(
        "{}{}{}{}{}",
        base_url,
        "?client=gtx&ie=UTF-8&oe=UTF-8&dt=t",
        format!("{}{}", "&sl=", source),
        format!("{}{}", "&tl=", target),
        format!("&q={}", q)
    )
}
