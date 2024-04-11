use rocket::State;
use crate::services::riot::{Region, RiotService, SummonerResponse};

pub async fn get_summoner_from_riot_id(game_name: &str, tag_line: &str, region: Region, riot_service: &State<RiotService>) -> Result<SummonerResponse, reqwest::Error> {
    let puuid = riot_service.get_puuid_from_riot_id(game_name, tag_line, region).await?;
    let summoner = riot_service.get_summoner_from_puuid(puuid.as_str(), Region::EUW1).await?;

    Ok(summoner)
}

