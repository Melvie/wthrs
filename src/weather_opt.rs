
use std::error::Error;
use structopt::StructOpt;
use crate::config::Config;


#[derive(StructOpt, Debug)]
pub enum Command {
	#[structopt(name = "current")]
	Current,
	#[structopt(name = "forecast")]
	Forecast,
	#[structopt(name = "config")]
	Config 
}

#[derive(StructOpt, Debug)]
pub struct WeatherOpt {
	#[structopt(subcommand)] 
	pub cmd: Command,
	#[structopt(short="h", long)]
	pub city: Option<String>,
	#[structopt(short="b", long)]
	pub country: Option<String>,
	#[structopt(short="k", long)]
	pub api_key: Option<String>,
	#[structopt(short="d", long, default_value = "3")]
	pub days: i8

}

impl WeatherOpt {

	pub fn parse_args() -> Result<(), Box<dyn Error>> {
		let opt: WeatherOpt = Self::from_args();

		match opt.cmd {
			Command::Current => opt.get_current(),
			Command::Forecast => opt.get_current(),
			Command::Config => opt.get_current(),
		}
	}

	fn get_current(&self) -> Result<(), Box<dyn Error>> {
		println!("{:?}", self);
		Ok(())
	}

	fn get_forecast(self) ->  Result<(), Box<dyn Error>>{
		println!("{:?}", self);
		Ok(())
	}

	fn setup_config(&self)  -> Result<(), Box<dyn Error>> {
		Config::new(&self)
	}

}