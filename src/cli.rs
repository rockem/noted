use clap::{Parser, Subcommand};
use noted::version::VERSION;

#[derive(Parser, Debug)]
#[command(name = "noted", version = VERSION)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(alias = "d", about = "Create or edit today's daily note")]
    Daily(DailyArgs),
}

#[derive(Parser, Debug)]
pub struct DailyArgs {
    #[arg(short = 'e', num_args = 0.., help = "Quick capture text. Use '-' to read from stdin.")]
    pub text: Option<Vec<String>>,
}
