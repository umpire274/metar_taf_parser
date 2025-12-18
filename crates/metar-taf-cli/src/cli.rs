use clap::{ArgGroup, Parser, Subcommand};

/// METAR & TAF parser CLI
#[derive(Parser, Debug)]
#[command(
    name = "metar-taf",
    version,
    about = "Fetch and parse METAR/TAF reports"
)]
pub struct Cli {
    /// ICAO airport code (e.g. LIRF, EGLL, KJFK)
    #[arg(value_name = "ICAO", global = true)]
    pub icao: Option<String>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Fetch METAR and/or TAF for an airport
    #[command(group(
        ArgGroup::new("what")
            .required(true)
            .args(&["metar", "taf", "all"])
    ))]
    Get {
        /// Fetch METAR report
        #[arg(long)]
        metar: bool,

        /// Fetch TAF report
        #[arg(long)]
        taf: bool,

        /// Fetch both METAR and TAF
        #[arg(long)]
        all: bool,
    },
}
