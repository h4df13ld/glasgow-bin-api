// use serde::{Serialize, Deserialize};
// use serde_json::json;
// use axum::{routing::get, Router, response::Html};
// use tower::ServiceBuilder;
// use tower_http::{trace::TraceLayer};

// #[derive(Debug, Serialize, Deserialize)]
// struct BinData {
//     colour: String,
//     due_tomorrow: bool,
// }

// #[tokio::main]
// async fn main() {
//     let app = Router::new()
//         .route("/test", get(handler))
//         .route("/bins", get(get_bins))
//         .layer(
//             ServiceBuilder::new()
//                 .layer(TraceLayer::new_for_http())
//                 // .layer(Extension(State {}))
//         );

//     // let app = ServiceBuilder::new()
//     //     .layer(tower::limit::ConcurrencyLimitLayer::new(100))
//     //     .layer(tower_http::trace::TraceLayer::new_for_http())
//     //     .layer(tower_http::compression::CompressionLayer::new())
//     //     .service(app);

//     axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
//         .serve(app.into_make_service())
//         .await
//         .unwrap();
// }

// #[allow(dead_code)]
// async fn get_bins() -> axum::Json<Vec<BinData>> {
//     if let Ok(response) = reqwest::blocking::get(
//         "https://www.glasgow.gov.uk/forms/refuseandrecyclingcalendar/CollectionsCalendar.aspx?UPRN=906700382583"
//     ).and_then(|res| res.text())
//     {
//         let response = response.to_string(); // convert &str to String
//         let document = scraper::Html::parse_document(&response);
//         let bin_query = scraper::Selector::parse("li>p").unwrap();

//         let list_of_bins = document.select(&bin_query).map(|x| x.inner_html());

//         fn get_bin_colour(colour_string: &str) -> String {
//             match colour_string {
//                 _ if colour_string.contains("Blue") => "Blue".to_string(),
//                 _ if colour_string.contains("Brown") => "Brown".to_string(),
//                 _ if colour_string.contains("Green") => "Green".to_string(),
//                 _ if colour_string.contains("Purple") => "Purple".to_string(),
//                 _ => "Unknown".to_string(),
//             }
//         }

//         let bins_due_tomorrow: Vec<BinData> = list_of_bins
//             .map(|bin_string| BinData {
//                 colour: get_bin_colour(&bin_string),
//                 due_tomorrow: bin_string.contains("Tomorrow"),
//             })
//             .collect();

//         println!("1");
//         axum::Json(bins_due_tomorrow)
//     } else {
//         println!("2");
//         axum::Json(vec![])
//     }
// }

extern crate rocket;


use rocket::serde::json::Json;
use rocket::serde::json::Value;
use rocket::serde::{Serialize, Deserialize};
use rocket::{launch, routes};
use rocket::get;
use std::collections::HashMap;

#[derive(Serialize, Debug)]
#[serde(crate = "rocket::serde")]
struct BinData {
    colour: String,
    due_tomorrow: bool,
}


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

        // println!("{:?}", list_of_bins);

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

        println!("{:?}", collection_status);

        Some(serde_json::json!(collection_status))
    } else {
        Some(serde_json::json!("Error"))
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![get_bins])
}






// fn main() {
//     if let Ok(response) = reqwest::blocking::get(
//         "https://www.glasgow.gov.uk/forms/refuseandrecyclingcalendar/CollectionsCalendar.aspx?UPRN=906700382583"
//     ).and_then(|res| res.text())
//     {
//         let response = response.to_string();
//         let document = scraper::Html::parse_document(&response);
//         let bin_query = scraper::Selector::parse("li>p").unwrap();

//         let list_of_bins = document.select(&bin_query).map(|x| x.inner_html());

//         fn get_bin_colour(colour_string: &str) -> String {
//             match colour_string {
//                 _ if colour_string.contains("Blue") => "Blue".to_string(),
//                 _ if colour_string.contains("Brown") => "Brown".to_string(),
//                 _ if colour_string.contains("Green") => "Green".to_string(),
//                 _ if colour_string.contains("Purple") => "Purple".to_string(),
//                 _ => "Unknown".to_string(),
//             }
//         }

//         let bins_due_tomorrow: Vec<BinData> = list_of_bins
//             .map(|bin_string| BinData {
//                 colour: get_bin_colour(&bin_string),
//                 due_tomorrow: bin_string.contains("Saturday"),
//             })
//             .collect();

//         println!("{:?}", bins_due_tomorrow);
//         // Json(bins_due_tomorrow)
//     } else {
//         // Json(vec![])
//         println!("Fail")

//     }
// }






// fn main() {

//     let response = reqwest::blocking::get(
//         "https://www.glasgow.gov.uk/forms/refuseandrecyclingcalendar/CollectionsCalendar.aspx?UPRN=906700382583"
//     )
//     .unwrap()
//     .text()
//     .unwrap();

//     let document = scraper::Html::parse_document(&response);
//     let bin_query = scraper::Selector::parse("li>p").unwrap();

//     let list_of_bins = document.select(&bin_query).map(|x| x.inner_html());

//     fn get_bin_colour(colour_string: &String) -> String {
//         let return_colour = match() {
//             _ if colour_string.contains("Blue") => "Blue".to_string(),
//             _ if colour_string.contains("Brown") => "Brown".to_string(),
//             _ if colour_string.contains("Green") => "Green".to_string(),
//             _ if colour_string.contains("Purple") => "Purple".to_string(),
//             _ => "Unknown".to_string()
//         };
//         return_colour
//     }


//     let bins_due_tomorrow: Vec<(String, bool)> = list_of_bins
//         .map(|bin_string| (get_bin_colour(&bin_string), bin_string.as_str().contains("Tomorrow")))
//         .collect();

//     println!("{:?}", bins_due_tomorrow);

//     let json_data: Vec<_> = bins_due_tomorrow
//         .into_iter()
//         .map(|(colour, due_tomorrow)| BinData {colour, due_tomorrow})
//         .map(|bin_data| json!(bin_data))
//         .collect();

//     println!("{:?}", json_data);

//     let json_string = serde_json::to_string_pretty(&json_data).unwrap();

    // println!("{}", json_string)


    // let blue_bin_due: bool = list_of_bins[0].contains("Tomorrow");
    // println!("{:?}", &list_of_bins)



// }
