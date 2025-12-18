/*use metar_taf_core::parse_metar;

fn main() {
    //let metar = "LIRF 121250Z 18010KT 5000 2000SW FEW030 18/12 Q1015";
    let metar = "LIRF 121250Z 18010KT 5000 2000SW FEW030 SCT050CB BKN100 OVC/// 18/12 Q1015";
    //let metar = "LIRF 121250Z 18010KT 5000 2000SW FEW030 SCT050 BKN100 18/12 Q1015";
    //let metar = "LIMC 121250Z 02005KT 9999 SCT020 M05/M10 Q1020";
    //let metar = "LIRF 121250Z 18010KT 9999 FEW030 18/12 Q1015";
    //let metar = "KJFK 121251Z 22015KT 10SM SCT020 22/18 A2992";
    //let metar = "LIRF 121250Z 18010KT 9999 -RA BR FEW030 18/12 Q1015";
    //let metar = "LIMC 121250Z 02005KT TSRA SCT020 BKN080 20/18 Q1008";
    //let metar = "LIRF 121250Z 18010KT 5000 2000SW FEW030 18/12 Q1015";
    //let metar="LIRF 121250Z 18010KT 5000 2000SW FEW030 SCT050CB BKN100 OVC/// 18/12 Q1015";
    //let metar = "LIRF 121250Z 18010KT 9999 -RA BR FEW030 18/12 Q1015";

    match parse_metar(metar) {
        Ok(parsed) => println!("{:#?}", parsed),
        Err(e) => eprintln!("Error: {}", e),
    }
}*/

mod cli;
mod commands;
mod fetch;
mod input;

use clap::Parser;
use cli::Cli;

fn main() {
    let cli = Cli::parse();
    commands::execute(cli.command, cli.icao);
}
