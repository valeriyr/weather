mod weather_mod;

use clap::{ArgEnum, Parser, Subcommand};

/// Could be the best weather provider but just a simple application for the Rust language training...
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Sets configuration from the terminal.
    #[clap(arg_required_else_help = true)]
    Configure {
        /// Provider
        #[clap(arg_enum)]
        provider: Provider,
    },
    /// Gets weather.
    #[clap(arg_required_else_help = true)]
    Get {
        /// The city
        city: String,

        /// Date in Year-Month-Day format
        #[clap(default_value_t = String::from(weather_mod::CURRENT_DATE_NAME))]
        date: String,
    },
}

#[derive(Debug, Clone, ArgEnum)]
enum Provider {
    OW,
    WAPI,
}

impl From<Provider> for weather_mod::Provider {
    fn from(provider: Provider) -> weather_mod::Provider {
        match provider {
            Provider::OW => weather_mod::Provider::OpenWeather,
            Provider::WAPI => weather_mod::Provider::WeatherAPI,
        }
    }
}

fn main() {
    let args = Args::parse();

    let mut weather = weather_mod::Weather::new();

    match args.command {
        Commands::Configure { provider } => {
            weather.configure(weather_mod::Provider::from(provider));
        }
        Commands::Get { city, date } => {
            weather.get_weather(&city, &date);
        }
    }
}
