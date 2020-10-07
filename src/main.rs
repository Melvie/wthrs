use anyhow::Error;
pub mod weather_opt;

use structopt::StructOpt;

fn main() -> Result<(), Error> {
    let mut opt = weather_opt::WeatherOpt::from_args();
    opt.cmd.run()
}
