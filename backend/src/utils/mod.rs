use reqwest::{Client, Url};
use rocket::serde::DeserializeOwned;
use crate::services::riot::Region;

pub fn create_url(region: Region, endpoint: String) -> Url {
    let url = format!("https://{}.api.riotgames.com{}", region, endpoint);

    Url::parse(&*url).expect("url must be valid")
}

pub async fn request<T: DeserializeOwned>(url: Url, http_client: &Client) -> Result<T, reqwest::Error> {
    let request = http_client.get(url);
    let response = request.send().await?.json::<T>().await?;

    Ok(response)
}
