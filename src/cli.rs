
use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about)]
pub struct Cli {
    pub name: Option<String>,

    #[arg(long)]
    pub list: bool,

    #[arg(long)]
    pub id: Option<u16>
}
