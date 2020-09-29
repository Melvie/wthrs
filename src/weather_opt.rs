
use anyhow::Error;
use structopt::StructOpt;
use confy::{ConfyError};
use serde::{Serialize, Deserialize};

const END_POINT: &str = "https://api.weatherbit.io/v2.0/";

#[derive(StructOpt, Debug)]
pub enum Command {
	#[structopt(name = "current")]
	Current(CmdData),
	#[structopt(name = "forecast")]
	Forecast(CmdData),
	#[structopt(name = "config")]
	Config(CmdData)
}

#[derive(StructOpt, Default, Debug, Serialize, Deserialize)]
pub struct CmdData {
	#[structopt(short="h", long)]
	city: Option<String>,
	#[structopt(short="b", long)]
	country: Option<String>,
	#[structopt(short="k", long)]
	api_key: Option<String>,
	#[structopt(short="d", long, default_value = "3")]
	days: i8

}

#[derive(StructOpt, Debug)]
pub struct WeatherOpt {
	#[structopt(subcommand)] 
	pub cmd: Command,
}


impl Command {
	// #[unreachable!()]
	pub fn data(&self) -> &CmdData {
		match self {
			Command::Current(data) => data,
			Command::Forecast(data) => data,
			Command::Config(data) => data
		}
	}

	pub fn run(&self) -> Result<(), Error> {
		match self {
			Command::Current(_data) => self.get_curret_weather(),
			Command::Forecast(_data) => self.get_weather_forecast(),
			Command::Config(_data) => self.set_config()
		}
	}

	fn get_curret_weather(&self) -> Result<(), Error> {
		//get config & fill in blanks1
		// let default = self.get_config()?;
		// let mut data = self.data();
		// if data.city.is_none() || data.country.is_none() {
		// 	data.city = default.city; // ????
		// 	data.country = default.country;
		// }

		// data.api_key = default.api_key;

		println!("Fetching weather..");
		Ok(())
	}

	fn get_weather_forecast(&self) -> Result<(), Error> {
		println!("Fetching forecast...");
		Ok(())
	}

	fn set_config(&self) -> Result<(), Error> {
		confy::store("weather_config",self.data())?;
		Ok(())
	}

	fn get_config(&self) -> Result<CmdData, ConfyError> {

		let default_config: CmdData = confy::load("weather_config")?;
		Ok(default_config)
	}

	// fn save_config(&self)
}