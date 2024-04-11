use rocket::{http::Status, serde::json::Json, State};
use crate::{helpers, services::riot::{RiotService, SummonerResponse}, utils};

#[get("/summoner/<server>/<game_name>/<tag_line>")]
pub async fn summoner_by_riot_id(game_name: &str, tag_line: &str, server: &str, riot_service: &State<RiotService>) -> Result<Json<SummonerResponse>, Json<Status>> {
    let region = utils::get_region_from_server_string(server);
    let res = helpers::get_summoner_from_riot_id(game_name, tag_line, region, &riot_service).await;

    match res {
        Ok(summoner) => Ok(Json(summoner)),
        Err(_err) => {        
            Err(Json(Status::NotFound))
        }
    }
}
