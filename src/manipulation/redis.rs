use std::env;
use serde_json;
use super::{URedisPayload, URedisResponse};

pub fn save_redis(value: String) ->Result<(), u32> {
    let uredis_url = env::var("uREDIS_URL").expect("uREDIS_URL not set");
    let url = format!("{uredis}/api/cache", uredis=uredis_url);
    let payload: URedisPayload = URedisPayload::new("dasdasdads", value);
    let client = reqwest::blocking::Client::new();
    let resp = client.post(url)
        .json(&payload)
        .send();

    // println!("{:?}", resp);
    
    match resp {
        Ok(response) => {
            println!("{:?}", response.status());
            if response.status() == 200 {
                ok(())
            } else {
                Err(response.status())
            }
            // println!("{:?}", response.json::<serde_json::Value>().unwrap());
            Ok(())
        },
        Err(_error) => Err(500u32),
    }
    // ()
}