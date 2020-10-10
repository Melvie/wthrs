use anyhow::Error;
use serde::{Deserialize, Serialize};
use crate::weather_opt::CmdData;
use reqwest::Url;
use std::fmt;


#[derive(Debug, Deserialize, Serialize)]
struct Weather {
	icon: String,
	code: i32,
	description: String
}

#[derive(Debug, Deserialize, Serialize)]
struct DailyWeather {
	valid_date: String,
	ts: i32,
	datetime: String,
	wind_gust_spd: f32,
	wind_spd: f32,
	wind_dir: f32,
	wind_cdir: String,
	wind_cdir_full: String,
	temp: f32,
	max_temp: f32,
	min_temp: f32,
	high_temp: f32,
	low_temp: f32,
	app_max_temp: f32,
	app_min_temp: f32,
	pop: f32,
	precip: f32,
	snow: f32,
	snow_depth: f32,
	slp: f32,
	pres: f32,
	dewpt: f32,
	rh: f32,
	weather: Weather,
	clouds_low: f32,
	clouds_mid: f32,
	clouds_hi: f32,
	clouds: f32,
	vis: f32,
	max_dhi: Option<i32>,
	uv: f32,
	moon_phase: f32,
	moon_phase_lunation: f32,
	moonrise_ts: i32,
	moonset_ts: i32,
	sunrise_ts: i32,
	sunset_ts: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Forecast {
	data: Vec<DailyWeather>,
	city_name: String,
	lon: String,
	timezone: String,
	lat: String,
	country_code: String,
	state_code: String
}

impl DailyWeather {
	fn feels_like(&self) -> f32 {
		(self.app_max_temp + self.app_min_temp)/2.0
	}
}

impl Forecast {
	pub async fn get(query_data: &CmdData) -> Result<Self, Error>{
		let url: String = format!(
			"https://api.weatherbit.io/v2.0/forecast/daily?city={}&country={}&days={}&key={}", 
			query_data.city, query_data.country, query_data.days, query_data.api_key);

			let url: Url = Url::parse(&*url)?;

			let resp = reqwest::get(url)
				.await?
				.json::<Forecast>()
				.await?;

			Ok(resp)
	}
}

impl fmt::Display for Forecast {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "#################################################\n\n")?;
		write!(f, "Todays weather for {},{}: \n", self.city_name, self.country_code)?;
		for day in &self.data {
			write!(f, "	{}\n", day.weather.description)?;
			write!(f, "	Current Temp: {}\n", day.temp)?;
			write!(f, "	Feels like: {}\n", day.feels_like())?;
			write!(f, "	High/Low: {}, {}\n", day.high_temp, day.low_temp)?;
			write!(f, "	POP%: {}\n", day.pop)?;
			write!(f, "------------------------------------\n")?;
		}
		write!(f, "\n#################################################")?;
		Ok(())
	}
}