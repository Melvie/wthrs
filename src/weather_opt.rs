use anyhow::Error;
use confy::ConfyError;
use serde::{Deserialize, Serialize};
use structopt::StructOpt;
use crate::weather::Forecast;

#[derive(StructOpt, Debug)]
pub enum Command {
    #[structopt(name = "forecast")]
    Forecast(CmdData),
    #[structopt(name = "config")]
    Config(CmdData),
}

pub enum Response {
    RequestResponse(Forecast),
    ConfigSuccessResponse
}
#[derive(StructOpt, Default, Debug, Serialize, Deserialize)]
pub struct CmdData {
    #[structopt(short = "h", long, default_value = "")]
    pub city: String,
    #[structopt(short = "b", long, default_value = "")]
    pub country: String,
    #[structopt(short = "k", long, default_value = "")]
    pub api_key: String,
    #[structopt(short = "d", long, default_value = "3")]
    pub days: i8,
}

#[derive(StructOpt, Debug)]
pub struct WeatherOpt {
    #[structopt(subcommand)]
    pub cmd: Command,
}

impl Command {
    fn data_as_mut(&mut self) -> &mut CmdData {
        match self {
            Command::Forecast(data) => data,
            Command::Config(data) => data,
        }
    }

    pub fn data(&self) -> &CmdData {
        match self {
            Command::Forecast(data) => data,
            Command::Config(data) => data,
        }
    }

    pub async fn run(&mut self) -> Result<Response, Error> {
        match self {
            Command::Forecast(_data) => self.get_weather_forecast().await,
            Command::Config(_data) => self.set_config(),
        }
    }

    pub async fn get_weather_forecast(&mut self) -> Result<Response, Error> {
        self.apply_defaults()?;
        let resp = Forecast::get(self.data()).await?;
        Ok(Response::RequestResponse(resp))
    }

    fn apply_defaults(&mut self) -> Result<(), Error> {
        let default = self.get_config()?;
        let mut data = self.data_as_mut();
        if data.city.is_empty() || data.country.is_empty() {
            data.city = default.city;
            data.country = default.country;
        }

        data.api_key = default.api_key;
        println!("Request Data: {:?}", data);
        Ok(())
    }

    fn set_config(&self) -> Result<Response, Error> {
        confy::store("weather_config", self.data())?;
        let resp = Ok(Response::ConfigSuccessResponse);
        resp
    }

    fn get_config(&self) -> Result<CmdData, ConfyError> {
        let default_config: CmdData = confy::load("weather_config")?;
        Ok(default_config)
    }

    // fn save_config(&self)
}
