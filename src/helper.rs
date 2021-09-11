use chrono::Local;

pub fn get_today() -> String {
    Local::today().format("%Y-%m-%d").to_string()
}
