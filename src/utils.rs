use std::{io::stdout, time::Duration};

use crossterm::{cursor::MoveTo, execute, terminal::{Clear, ClearType}};

use crate::models::Kota;

pub async fn spinner_loop(mgs: &str) {
    let mut spinner_index = 0;
    let spinner = ['|', '/', '-', '\\'];
    clear_line(0, 0);
    loop {
        tokio::time::sleep(Duration::from_millis(120)).await;
        print!("\r{} {}", spinner[spinner_index % spinner.len()], mgs);
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        spinner_index += 1;
    }
}

pub fn clear_line(x: u16, y: u16) {
    execute!(
        stdout(),
        Clear(ClearType::All),
        MoveTo(x,y)
    ).unwrap();
}

// Cek kota atau kabupaten yang tersedia
// Apakah lebih dari 2 atau tidak
pub fn is_multiple_city(city: &Vec<Kota>) -> bool {
    city.len() > 2
}
