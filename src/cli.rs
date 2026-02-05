
use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about)]
#[command(after_help="Example: \n  jsh maros\n  jsh --id 1001\n  jsh --list")]
pub struct Cli {
    /// Name of city
    pub name: Option<String>,

    #[arg(long)]
    /// Show all of city
    pub list: bool,

    #[arg(long)]
    /// ID of city, you can see id by run --list
    pub id: Option<u16>,

    /// To make selected city default
    /// Example: jsh --id 1001 -d, jsh malang -d, jsh --list -d 
    #[arg(long, short)]
    pub default: bool,

    /// Print prayer shedule without spinner or animation (work with --id only for now)
    #[arg(long, short)]
    pub simple: bool
}
