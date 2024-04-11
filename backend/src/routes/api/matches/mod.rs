use rocket::{http::Status, State, serde::json::Json}; 
use futures::future::join_all;
use crate::services::riot::{Region, RiotService};

#[get("/matches/<puuid>/<page>/<amount>")]
pub async fn match_data_by_puuid(
    puuid: &str,
    page: i32,
    amount: i32,
    riot_service: &State<RiotService>
    ) -> Result<Json<Vec<rocket::serde::json::Value>>, Status> {
    let match_ids = riot_service.get_match_ids_from_puuid(puuid, Region::EUROPE, amount, page).await;

    match match_ids {
        Ok(ids) => {
            let mut futures = Vec::new();

            for id in ids {
                let future = riot_service.get_match_from_id(id, Region::EUROPE);

                futures.push(future);
            }

            let mut matches = Vec::new();
            let res = join_all(futures).await;
            for match_data in res {
                matches.push(match_data.unwrap());
            }

            Ok(Json(matches))
        },
        Err(_err) => {
            Err(Status::InternalServerError)
        }
    }
}
