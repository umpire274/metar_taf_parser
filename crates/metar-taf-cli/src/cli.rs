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
    #[arg(
        value_name = "ICAO",
        help = "ICAO airport code (e.g. LIRF, EGLL, KJFK)"
    )]
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
        #[arg(long, help = "Fetch METAR report")]
        metar: bool,

        /// Fetch TAF report
        #[arg(long, help = "Fetch TAF report")]
        taf: bool,

        /// Fetch both METAR and TAF
        #[arg(long, help = "Fetch both METAR and TAF")]
        all: bool,

        /// Output parsed data as JSON
        #[arg(long, conflicts_with = "raw", help = "Output parsed data as JSON")]
        json: bool,

        /// Output raw METAR / TAF only
        #[arg(long, conflicts_with = "json", help = "Output raw METAR / TAF only")]
        raw: bool,
    },
}
