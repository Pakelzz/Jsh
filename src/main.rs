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

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let cli = Cli::parse();
    if let Some(city) = cli.name {
        run(&city, time::now(), cli.default).await;
    } else if let Some(city_id) = cli.id {
        run_by_id(city_id, time::now(), cli.default).await
    } else if cli.list {
        let mut terminal = ratatui::init();
        App::new().await.unwrap().run(&mut terminal)?;
        ratatui::restore();
    } else {
        let default_config = read_config();
        if default_config == 0 {
            println!("You have no config yet");
        } else {
            run_by_id(default_config, time::now(), false).await;
        }
    }

    Ok(())
}
