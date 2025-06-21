use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref LANG_TO_COLORS: HashMap<String, String> =
        serde_json::from_str(include_str!("../../data/lang2hex.json")).unwrap();
    pub static ref DEFAULT_LANG_COLOR: String = "#818181".to_string();
}

pub fn get_lang_color(lang_name: &String) -> String {
    match LANG_TO_COLORS.get(&lang_name.to_lowercase()) {
        None => DEFAULT_LANG_COLOR.clone(),
        Some(color) => color.clone(),
    }
}
