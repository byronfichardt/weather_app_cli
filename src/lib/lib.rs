use std::{collections::HashMap, fs};
use std::path::PathBuf;
use config::Config;

pub fn view(view: &str) -> PathBuf {
    let settings_value = config();
    let val = settings_value.get("views").unwrap();

    let mut viewfile = PathBuf::from(val);
    viewfile.push("404.html");

    for entry in fs::read_dir(val).unwrap() {
        let entry = entry.unwrap();
        if let Some(path) = entry.path().to_str() {
            if path.contains(view) {
                return entry.path();
            }
        }
    }
    viewfile
}

fn config () -> HashMap<String, String> {
    let settings = Config::builder()
        .add_source(config::File::with_name("src/config/config.ini"))
        .build()
        .unwrap();

    let settings_value = settings.try_deserialize::<HashMap<String, String>>().unwrap();
    settings_value
}