use anyhow::Error;
use tokio;
pub mod weather_opt;
mod weather;

use structopt::StructOpt;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut opt = weather_opt::WeatherOpt::from_args();
    let resp: weather_opt::Response = opt.cmd.run().await?;

    if let weather_opt::Response::RequestResponse(forecast) = resp {
        println!{"{}", forecast}
    }
    Ok(())

}
