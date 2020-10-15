use anyhow::Error;
use wthrs::weather_opt::WeatherOpt;

#[tokio::main]
async fn main() -> Result<(), Error> {
    WeatherOpt::parse_args().await?;
    Ok(())
}
