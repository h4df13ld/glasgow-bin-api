extern crate rocket;

use rocket::serde::json::Json;
use rocket::serde::json::Value;
use rocket::serde::{Serialize, Deserialize};
use rocket::{launch, routes};
use rocket::config::{Config};
use rocket::get;
use std::collections::HashMap;


#[get("/bins")]
async fn get_bins() -> Option<Value> {
    if let Ok(response) = reqwest::get(
        "https://www.glasgow.gov.uk/forms/refuseandrecyclingcalendar/CollectionsCalendar.aspx?UPRN=906700382583"
    ).await.and_then(|res| Ok(res.text()))
    {
        
        let response = response.await.unwrap().to_string();
        let document = scraper::Html::parse_document(&response);
        let bin_query = scraper::Selector::parse("li>p").unwrap();

        let list_of_bins = document.select(&bin_query).map(|x| x.inner_html());


        fn get_bin_colour(colour_string: &str) -> String {
            match colour_string {
                _ if colour_string.contains("Blue") => "Blue".to_string(),
                _ if colour_string.contains("Brown") => "Brown".to_string(),
                _ if colour_string.contains("Green") => "Green".to_string(),
                _ if colour_string.contains("Purple") => "Purple".to_string(),
                _ => "Unknown".to_string(),
            }
        }

        let collection_status: HashMap<String, bool> = list_of_bins
            .into_iter()
            .map(|bin_string| {
                (get_bin_colour(&bin_string), bin_string.contains("Tomorrow"))
            })
            .collect();

        Some(serde_json::json!(collection_status))
    } else {
        Some(serde_json::json!("Error"))
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![get_bins])
}
