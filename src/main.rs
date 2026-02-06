use clap::Parser;

use crate::{
    app::{run, run_by_id}, cli::Cli, storage::read_config, ui::App
};

mod api;
mod app;
mod cli;
mod helper;
mod models;
mod print;
mod time;
mod ui;
mod utils;
mod storage;
mod error;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let cli = Cli::parse();
    if let Some(city) = cli.name {
        run(&city, time::now(), cli.default, false).await;
    } else if let Some(city_id) = cli.id {
        run_by_id(city_id, time::now(), cli.default, cli.simple).await;
    } else if cli.list {
        let mut terminal = ratatui::init();
        match App::new(cli.default).await {
            Ok(mut app) => {
                app.run(&mut terminal)?;
                ratatui::restore();
            },
            Err(e) => {
        ratatui::restore();
                eprintln!("Error: {e}");
            }
        };
    } else {
        let default_config = read_config();
        match default_config {
            Ok(id) => {
                if id == 0 {
                    eprintln!("You have no config yet");
                } else {
                    run_by_id(id, time::now(), false, cli.simple).await;
                }
            },
            Err(e) => eprintln!("Error: {e}")
        };
    }

    Ok(())
}
