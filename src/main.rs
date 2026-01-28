use clap::Parser;

use crate::{
    app::{run, run_by_id},
    cli::Cli, ui::App,
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

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let cli = Cli::parse();
    if let Some(city) = cli.name {
        run(&city, time::now()).await;
    }
    else if let Some(city_id) = cli.id {
        run_by_id(city_id, time::now()).await
    }
     else if cli.list {
        let mut terminal = ratatui::init();
        let result = App::new().await.unwrap().run(&mut terminal)?;
        ratatui::restore();
        result
    }

    Ok(())
}
