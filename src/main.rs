use chrono::{NaiveDate};
use clap::{Parser, Subcommand, ArgEnum};

/// Could be the best weather provider but just a simple application for the Rust language training...
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    command: Commands
}

#[derive(Debug, Subcommand)]
enum Commands
{
    /// Sets configuration from the terminal.
    #[clap(arg_required_else_help = true)]
    Configure {
        /// Provider
        #[clap(arg_enum)]
        provider: Proveder,
    },
    #[clap(arg_required_else_help = true)]
    Get {
        /// Address
        address: String,

        // Date in Year-Month-Day format
        #[clap(default_value_t = String::from("NOW"))]
        date: String
    }
}

#[derive(Debug, Clone, ArgEnum)]
enum Proveder
{
    Provider1,
    Provider2
}

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Configure { provider } => {
            println!("Configuring {:?}", provider);
        }
        Commands::Get { address, date } => {
            println!("Getting {}, {}", address, date);

            let dt1 = NaiveDate::parse_from_str(&date, "%Y-%m-%d");

            println!("{:?}", dt1);
        }
    }
}
