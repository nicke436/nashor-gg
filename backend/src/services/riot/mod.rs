use reqwest::{header::{HeaderMap, HeaderValue}, Client};
use rocket::serde::{Deserialize, Serialize};
use crate::utils;

#[derive(Debug)]
pub struct RiotService {
    pub http_client: Client,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum Region {
    EUW1,
    BR1,
    EUN1,
    JP1,
    KR1,
    LA1,
    LA2,
    NA1,
    OC1,
    PH2,
    RU,
    SG2,
    TH2,
    TR1,
    TW2,
    VN2,
    AMERICAS,
    EUROPE,
    ASIA,
    ESPORTS,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AccountResponse {
    pub puuid: String,
    pub game_name: String,
    pub tag_line: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SummonerResponse {
    id: String,
    account_id: String,
    puuid: String,
    profile_icon_id: i32,
    revision_date: i64,
    summoner_level: i32,
}

impl std::fmt::Display for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl RiotService {
    pub fn new(api_key: String) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert("X-Riot-Token", HeaderValue::from_str(&api_key).expect("api key must be valid"));
        let http_client = Client::builder().default_headers(headers).build().unwrap();

        Self {
            http_client,
        }
    }

    pub async fn get_puuid_from_riot_id(&self, game_name: &str, tag_line: &str, region: Region) -> Result<String, reqwest::Error> {
        let endpoint = format!("/riot/account/v1/accounts/by-riot-id/{}/{}", game_name, tag_line);
        println!("url: {}", endpoint);
        let url = utils::create_url(region, endpoint);
        let request = self.http_client.get(url);

        let puuid = request.send()
            .await?.json::<AccountResponse>().await?.puuid;

        println!("{}", puuid);

        Ok(puuid)
   }

    pub async fn get_summoner_from_puuid(&self, puuid: &str, region: Region) -> Result<SummonerResponse, reqwest::Error> {
        let endpoint = format!("/lol/summoner/v4/summoners/by-puuid/{}", puuid);
        let url = utils::create_url(region, endpoint);

        let summoner_res = utils::request::<SummonerResponse>(url, &self.http_client).await?;

        Ok(summoner_res)
 
    }

    pub async fn get_match_ids_from_puuid(&self, puuid: &str, region: Region, amount: i32, page: i32) -> Result<Vec<String>, reqwest::Error>{
        let start = page * amount;
        let endpoint = format!("/lol/match/v5/matches/by-puuid/{}/ids?start={}&count={}", puuid, start, amount);

        let url = utils::create_url(region, endpoint);

        let request = self.http_client.get(url);

        let match_ids = request.send()
            .await?.json::<Vec<String>>().await?;

        Ok(match_ids)
    }

    pub async fn get_match_from_id(&self, id: String, region: Region) -> Result<rocket::serde::json::Value, reqwest::Error> {
        let endpoint = format!("/lol/match/v5/matches/{}", id);
        let url = utils::create_url(region, endpoint);

        let request = self.http_client.get(url);

        let match_res = request.send()
            .await?.json::<rocket::serde::json::Value>().await?;

        Ok(match_res)
    }
}
