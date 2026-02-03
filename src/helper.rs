use std::io::stdout;
use std::fmt::Write;

use crate::{api::get_jadwal, models::{ApiResponse, Client}, print::print_jadwal, storage::write_config, utils::{clear_line, is_multiple_city, spinner_loop}};


pub fn update_client(client: &mut Client, result: Result<ApiResponse, Box<dyn std::error::Error>>) {
    match result {
        Ok(response) => {
            while let Some(city) = &response.data {
                if is_multiple_city(city) {
                    let mut output = String::new();
                    write!(output, " ID      Lokasi\n").unwrap();
                    city.iter().for_each(|f| {
                        write!(output, "{}: {}\n", f.id, f.lokasi).unwrap();
                    });

                    write!(output, "Insert ID city: ").unwrap();
                    print!("{}", output);
                    std::io::Write::flush(&mut stdout()).unwrap();

                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input).unwrap();

                    client.id = Some(input.trim().to_string());
                    client.is_multiple = true;
                } else {
                    client.id = Some(city[0].id.to_string());
                }
                break;
            }
        }
        Err(_) => {
            client.id = None;
        }
    }
}

pub async fn output(client: Client, time: &str, make_default: bool) {
    if let Some(city_id) = client.id {
        let result = tokio::select! {
            res = get_jadwal(&city_id, time) => res,
            _ = spinner_loop("Loading prayer schedule ") => unreachable!(),
        };

        clear_line(0, 0);
        
        match result {
            Ok(jadwal_response) => {
                match jadwal_response.data {
                    Some(jadwal) => {
                        print_jadwal(jadwal, client.is_multiple);
                        if make_default {
                            write_config(city_id);
                        }
                    }
                    None => {
                        eprintln!("Please insert a valid value");
                    }
                }
            }
            Err(e) => {
                eprintln!("{}", e);
            }
        }
    } else {
        eprintln!("Please insert valid city or regency");
    }
}
