
extern crate wthrs;
use wthrs::weather_opt::{WeatherOpt, Response};
use wthrs::weather::Forecast;
use tokio;
use anyhow::Error;

// use structopt::StructOpt;

#[tokio::main]
async fn main() -> Result<(), Error> {

    let mut opt = WeatherOpt::from_args();
    let resp: Response = opt.cmd.run().await?;

    match resp {
       Response::RequestResponse(forecast) => println!("{:}", forecast),
       Response::ConfigSuccessResponse => println!("Success! Config saved.")
    }

    Ok(())

}
