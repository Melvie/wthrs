
extern crate wthrs;
use wthrs::weather_opt::WeatherOpt;
// use tokio;
use anyhow::Error;

// use structopt::StructOpt;

#[tokio::main]
async fn main() -> Result<(), Error> {
    WeatherOpt::parse_args().await?;
    Ok(())

}
