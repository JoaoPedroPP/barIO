use std::env;
// use serde_json;
use super::URedisPayload;

pub fn save_redis(key: &str, value: String) ->Result<(), u16> {
    let uredis_url = env::var("uREDIS_URL").expect("uREDIS_URL not set");
    let url = format!("{uredis}/api/cache", uredis=uredis_url);
    let payload: URedisPayload = URedisPayload::new(key, value);
    let client = reqwest::blocking::Client::new();
    let resp = client.post(url)
        .json(&payload)
        .send();
    
    match resp {
        Ok(response) => {
            if response.status() == 201 {
                Ok(())
            } else {
                Err(response.status().as_u16())
            }
        },
        Err(_error) => Err(500u16),
    }
}