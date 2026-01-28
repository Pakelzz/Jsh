use chrono::Local;

pub fn now() -> String {
    Local::now().date_naive().to_string()
}

pub fn get_time() -> String {
    let local = Local::now();
    format!("{}", local.format("%H:%M:%S"))
}
