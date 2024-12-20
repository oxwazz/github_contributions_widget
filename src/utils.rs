use base64::engine::general_purpose;
use base64::Engine;
use chrono::{DateTime, Utc};
use std::time::Duration;

pub(crate) fn uppercase_first_letter(word: &str) -> String {
    let mut c = word.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().chain(c).collect(),
    }
}

pub(crate) fn get_formatted_date_now() -> String {
    Utc::now().format("%Y-%m-%d %H:%M:%S %Z").to_string()
}

pub(crate) fn parse_time_ago(timestamp: &str) -> String {
    let date1: DateTime<Utc> = match timestamp.parse() {
        Err(_) => return String::new(),
        Ok(v) => v,
    };
    let date2 = Utc::now();
    let seconds_diff = (date2 - date1).num_seconds();
    timeago::Formatter::new().convert(Duration::from_secs(seconds_diff as u64))
}

pub(crate) fn parse_number_compact(number: i32) -> String {
    let abs_number = number.abs();
    if abs_number < 1_000 {
        return number.to_string();
    }
    let suffixes = ["", "k", "M", "B", "T"];
    let log_number = (abs_number as f64).log10() / 3.0;
    let index = log_number.floor() as usize;
    if index >= suffixes.len() {
        return number.to_string();
    }
    let scaled_number = abs_number as f64 / (1_000_f64.powi(index as i32));
    let formatted = format!("{:.1}", scaled_number)
        .trim_end_matches(".0")
        .to_string();
    let sign = if number < 0 { "-" } else { "" };
    format!("{}{}{}", sign, formatted, suffixes[index])
}

pub(crate) async fn get_photo_base64_from_url(image_url: &str) -> String {
    let response = match reqwest::get(image_url).await {
        Err(_) => return String::new(),
        Ok(v) => v,
    };
    if !response.status().is_success() {
        return String::new();
    }
    let image_bytes = match response.bytes().await {
        Err(_) => return String::new(),
        Ok(v) => v,
    };
    general_purpose::STANDARD.encode(&image_bytes)
}
