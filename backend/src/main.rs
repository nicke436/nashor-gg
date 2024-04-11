#[macro_use] extern crate rocket;
use dotenv;
use routes::api::{summoner, matches};
mod routes;
mod services;
mod utils;
mod helpers;

use services::riot::RiotService;

#[get("/")]
fn index() -> String {
   String::from("Hello, World!")
}

#[launch]
fn rocket() -> _ {
    dotenv::dotenv().ok();
    let api_key = dotenv::var("API_KEY").expect("api key must be present");
    let riot_service = RiotService::new(api_key);

    println!("{:?}", riot_service);

    rocket::build()
        .manage(riot_service)
        .mount("/", routes![index])
        .mount("/api/v1/", routes![
               summoner::summoner_by_riot_id,
               matches::match_data_by_puuid,
        ]) 
}
