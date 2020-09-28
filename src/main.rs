pub mod weather_opt;
pub mod config;

use weather_opt::WeatherOpt;

fn main() {
    WeatherOpt::parse_args();
}
