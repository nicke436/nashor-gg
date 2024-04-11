use reqwest::{Client, Url};
use rocket::serde::DeserializeOwned;

use crate::services::riot::Region;

pub fn create_url<T: std::fmt::Display>(region: T, endpoint: String) -> Url {
    let url = format!("https://{}.api.riotgames.com{}", region, endpoint);

    Url::parse(&*url).expect("url must be valid")
}

pub async fn request<T: DeserializeOwned>(url: Url, http_client: &Client) -> Result<T, reqwest::Error> {
    let request = http_client.get(url);
    let response = request.send().await?.json::<T>().await?;

    Ok(response)
}

pub fn get_region_from_server_string(server_string: &str) -> Region {
    match server_string {
        "EUW1" => Region::EUROPE,
        "EUN1" => Region::EUROPE,
        "RU" => Region::EUROPE,
        "TR1" => Region::EUROPE,
        "JP1" => Region::ASIA,
        "KR1" => Region::ASIA, 
        "PH2" => Region::ASIA,
        "SG2" => Region::ASIA,
        "TH2" => Region::ASIA,
        "TW2" => Region::ASIA,
        "VN2" => Region::ASIA,
        "NA1" => Region::AMERICAS,
        "BR1" => Region::AMERICAS,
        "LA1" => Region::AMERICAS,
        "LA2" => Region::AMERICAS,
        "OC1" => Region::SEA,
        _ => Region::EUROPE,
    }
}


