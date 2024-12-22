use reqwest::{Client, Error};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AstronomyPictureDay {
    // pub _date: String,
    pub explanation: String,
    pub hdurl: String,
    // pub _media_type: String,
    // pub _service_version: String,
    pub title: String,
    // pub _url: String,
}

pub async fn get_astronomy_picture_day(api_key: String) -> Result<AstronomyPictureDay, Error> {
    let client = Client::new();

    let url = "https://api.nasa.gov/planetary/apod";
    let params = [("api_key", api_key)];

    let response = client.get(url).query(&params).send().await;

    match response {
        Ok(res) => {
            let data = res.json::<AstronomyPictureDay>().await?;

            Ok(data)
        }
        Err(err) => return Err(err),
    }
}
