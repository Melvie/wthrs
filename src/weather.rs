use crate::weather_opt::CmdData;
use anyhow::{format_err, Error, Result};
use reqwest::Url;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::collections::HashMap;
use structopt::lazy_static::lazy_static;

lazy_static! {
    static ref WEATHERICONS: HashMap<i32, &'static str> = {
        let mut map = HashMap::new();
        map.insert(800, "☀");
        map.insert(200, "⛈");
        map.insert(201, "⛈");
        map.insert(202, "⛈");
        map.insert(230, "🌩");
        map.insert(230, "🌩");
        map.insert(232, "🌩");
        map.insert(233, "🌩");
        map.insert(300, "🌧");
        map.insert(301, "🌧");
        map.insert(302, "🌧");
        map.insert(500, "🌧");
        map.insert(501, "🌧");
        map.insert(511, "🌧");
        map.insert(501, "🌧");
        map.insert(520, "🌧");
        map.insert(522, "🌧");
        map.insert(900, "🌧");
        map.insert(521, "🌦");
        map.insert(600, "🌨");
        map.insert(601, "🌨");
        map.insert(602, "🌨");
        map.insert(610, "🌨");
        map.insert(611, "🌨");
        map.insert(612, "🌨");
        map.insert(621, "🌨");
        map.insert(622, "🌨");
        map.insert(623, "🌨");
        map.insert(700, "🌨");
        map.insert(711, "🌫");
        map.insert(721, "🌫");
        map.insert(731, "🌫");
        map.insert(741, "🌫");
        map.insert(751, "🌫");
        map.insert(801, "🌤");
        map.insert(802, "🌥");
        map.insert(803, "⛅");
        map.insert(804, "☁");
        map
    };
}

#[derive(Debug, Deserialize, Serialize)]
struct Weather {
    icon: String,
    code: i32,
    description: String,
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
    state_code: String,
}

impl DailyWeather {
    fn feels_like(&self) -> f32 {
        (self.app_max_temp + self.app_min_temp) / 2.0
    }
}

impl Forecast {
    pub async fn get(query_data: &CmdData) -> Result<Self, Error> {
        let url: String = format!(
            "https://api.weatherbit.io/v2.0/forecast/daily?city={}&country={}&days={}&key={}",
            query_data.city, query_data.country, query_data.days, query_data.api_key
        );

        let url: Url = Url::parse(&*url)?;
        //TODO: Catch city/canada/location not found
        let resp = reqwest::get(url).await?.json::<Forecast>().await;

        match resp {
            Ok(resp) => Ok(resp),
            Err(_) => Err(format_err!(
                "Bad Request: Can't find weather for that location :/"
            )),
        }
    }
}

impl fmt::Display for Forecast {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#################################################\n\n")?;
        writeln!(
            f,
            "Today's weather for {},{}: ",
            self.city_name, self.country_code
        )?;
        writeln!(f, "{}\n", self.data[0])?;
        writeln!(f, "Forecast for next: {} day(s)", self.data.len())?;
        for day in &self.data[1..] {
            writeln!(f, "------------------------------------")?;
            writeln!(f, "Date: {}", day.datetime)?;
            writeln!(f, "{}", day)?;
        }

        write!(f, "#################################################")?;
        Ok(())
    }
}

impl fmt::Display for DailyWeather {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let weather = &self.weather;
        write!(
            f, 
            "{} {}\nCurrent Temp: {}\nFeels like: {}\nHigh/Low: {}/{}\nPOP%: {}",
            self.weather.description,
            WEATHERICONS[&weather.code],
            self.temp,
            self.feels_like(),
            self.high_temp,
            self.low_temp,
            self.pop
        )?;
        Ok(())
    }
}
