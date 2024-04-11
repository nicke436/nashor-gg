use rocket::{http::Status, serde::json::Json, State};
use crate::{services::riot::{Region, RiotService, SummonerResponse}, helpers};

#[get("/summoner/<game_name>/<tag_line>")]
pub async fn summoner_by_riot_id(game_name: &str, tag_line: &str, riot_service: &State<RiotService>) -> Result<Json<SummonerResponse>, Json<Status>> {
    let res = helpers::get_summoner_from_riot_id(game_name, tag_line, Region::EUROPE, &riot_service).await;

    match res {
        Ok(summoner) => Ok(Json(summoner)),
        Err(_err) => {        
            Err(Json(Status::NotFound))
        }
    }
}
