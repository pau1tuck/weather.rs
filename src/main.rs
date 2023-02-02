use std::env;

const API_KEY: &str = "ASRQL55QSXYKL63Y2DCEMFUC3";

#[tokio::main]

async fn main() -> Result<(), reqwest::Error> {
    let key: &str = env::var("KEY");
    let url = format!("https://weather.visualcrossing.com/VisualCrossingWebServices/rest/services/timeline/Mae%20Chan?unitGroup=metric&include=current&key={}&contentType=json", API_KEY);

    let weather_data = reqwest::Client::new().get(url).send().await?.text().await?;
    println!("{:#?}", weather_data);

    Ok(())
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Weather {
    query_cost: Option<u32>,
    latitude: Option<f64>,
    longitude: Option<f64>,
    resolved_address: Option<String>,
    address: Option<String>,
    timezone: Option<String>,
    tzoffset: Option<i64>,
    days: Option<Vec<CurrentConditions>>,
    stations: Option<Stations>,
    current_conditions: Option<CurrentConditions>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrentConditions {
    datetime: Option<String>,
    #[serde(rename = "datetimeEpoch")]
    datetime_epoch: Option<i64>,
    temp: Option<f64>,
    feelslike: Option<f64>,
    humidity: Option<f64>,
    dew: Option<f64>,
    precip: Option<f64>,
    precipprob: Option<f64>,
    snow: Option<i64>,
    snowdepth: Option<i64>,
    preciptype: Option<Vec<String>>,
    windgust: Option<f64>,
    windspeed: Option<f64>,
    winddir: Option<f64>,
    pressure: Option<f64>,
    visibility: Option<f64>,
    cloudcover: Option<f64>,
    solarradiation: Option<f64>,
    solarenergy: Option<f64>,
    uvindex: Option<i64>,
    conditions: Option<Conditions>,
    icon: Option<String>,
    stations: Option<Vec<String>>,
    source: Option<Source>,
    sunrise: Option<String>,
    #[serde(rename = "sunriseEpoch")]
    sunrise_epoch: Option<i64>,
    sunset: Option<String>,
    #[serde(rename = "sunsetEpoch")]
    sunset_epoch: Option<i64>,
    moonphase: Option<f64>,
    tempmax: Option<f64>,
    tempmin: Option<f64>,
    feelslikemax: Option<f64>,
    feelslikemin: Option<f64>,
    precipcover: Option<f64>,
    severerisk: Option<i64>,
    description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Stations {
    #[serde(rename = "VTCT")]
    vtct: Option<Vtct>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Vtct {
    distance: Option<i64>,
    latitude: Option<f64>,
    longitude: Option<f64>,
    #[serde(rename = "useCount")]
    use_count: Option<i64>,
    id: Option<String>,
    name: Option<String>,
    quality: Option<i64>,
    contribution: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Conditions {
    Clear,
    #[serde(rename = "Partially cloudy")]
    PartiallyCloudy,
    #[serde(rename = "Rain, Partially cloudy")]
    RainPartiallyCloudy,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Source {
    #[serde(rename = "comb")]
    Comb,
    #[serde(rename = "fcst")]
    Fcst,
    #[serde(rename = "obs")]
    Obs,
}
