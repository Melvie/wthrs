use wthrs::weather_opt::WeatherOpt;
use anyhow::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    WeatherOpt::parse_args().await?;
    Ok(())

}
