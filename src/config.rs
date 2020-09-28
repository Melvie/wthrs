use std::error::Error;
use std::path::Path;
use std::fs::File;
use std::io::Write;
use crate::weather_opt::WeatherOpt;
use crate::weather_opt::Command;
use dotenv;


const END_POINT: &str = "https://api.weatherbit.io/v2.0/";


pub struct Config;

impl Config {
	pub fn new(opt: &WeatherOpt) -> Result<(), Box<dyn Error>> {
		let config_dir = dirs::config_dir().unwrap();
		let config_path =  config_dir.join("wthrs").join("config.env");
		let config_file = Path::new(&config_path);

		let mut file = File::create(config_file)?;
		// let config_data: Config  = opt.cmd;

		file.write_fmt(format_args!("city={}", opt.city.as_ref().unwrap()))?;
		file.write_fmt(format_args!("country={}", opt.country.as_ref().unwrap()))?;
		file.write_fmt(format_args!("api_key={}", opt.api_key.as_ref().unwrap()))?;

		Ok(())
	}

	// #[unreachable!]
	// fn get_config() -> Result<Self, Box<dyn Error>> {
	// 	let config_dir = dirs::config_dir().ok_or_else(|| Err("No CONFIG dir."))?;
	// 	let config_path =  config_dir.join("wthrs").join("config.env");
	// 	let config_file = Path::new(config_path);

	// 	dotenv::dotenv().expect("Failed to read .env file");
	// 	match envy::from_env::<Config>() {
	// 		Ok(config) => config,
	// 		Err(e) => e,
	// 	}
	// }
}