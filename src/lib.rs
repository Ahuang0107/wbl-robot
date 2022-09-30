use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::mem::MaybeUninit;
use std::sync::Once;

pub fn env_get(key: &str) -> Option<&'static String> {
    load_env().get(key)
}

fn load_env() -> &'static HashMap<String, String> {
    static mut SINGLETON: MaybeUninit<HashMap<String, String>> = MaybeUninit::uninit();
    static ONCE: Once = Once::new();
    unsafe {
        ONCE.call_once(|| {
            let file = File::open(".env").unwrap_or_else(|_| {
                panic!("unable to find .env file");
            });
            let lines = BufReader::new(file).lines();
            let mut vars: HashMap<String, String> = HashMap::new();
            lines.for_each(|line| {
                let str = line.unwrap_or("".to_string());
                if !str.starts_with("#") && !str.is_empty() {
                    let (key, value) = split_by(str, '=');
                    vars.insert(key, value);
                }
            });
            SINGLETON.write(vars);
        });
        &SINGLETON.assume_init_ref()
    }
}

fn split_by(str: String, delimit: char) -> (String, String) {
    let split_index = str.find(delimit).unwrap_or(str.len());
    let (key, value) = str.split_at(split_index);
    let real_value: String = value.chars().skip(1).collect();
    (key.to_string(), real_value.to_string())
}

pub fn cookies(str: &String) -> HashMap<String, String> {
    let mut result: HashMap<String, String> = HashMap::new();
    str.split(";").into_iter().for_each(|key_value| {
        let (key, value) = split_by(key_value.trim().to_string(), '=');
        result.insert(key, value);
    });
    result
}
