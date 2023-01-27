use std::env;
use rdkafka::{
    consumer::{BaseConsumer, Consumer},
    ClientConfig
};

mod manipulation;

fn main() {
    dotenv::dotenv().ok();

    let kafka_servers = env::var("KAFKA_SERVERS").expect("KAFKA_SERVERS not set");
    let kafka_username = env::var("KAFKA_USERNAME").expect("KAFKA_USERNAME not set");
    let kafka_password = env::var("KAFKA_PASSWORD").expect("KAFKA_PASSWORD not set");
    let kafka_group = env::var("KAFKA_GROUP").expect("KAFKA_GROUP not set");
    let kafka_consumer_topic = env::var("KAFKA_CONSUMER_TOPIC").expect("KAFKA_CONSUMER_TOPIC not set");

    let consumer: BaseConsumer = ClientConfig::new()
        .set("bootstrap.servers", kafka_servers)
        .set("sasl.usernamer", kafka_username)
        .set("sasl.password", kafka_password)
        .set("security.protocol", "SASL_SSL")
        .set("sasl.mechanisms", "PLAIN")
        .set("group.id", kafka_group)
        .create()
        .expect("Invalid Kafka Consumer Config");

    consumer
        .subscribe(&[kafka_consumer_topic])
        .expect("Consumer topic failed");

    loop {
        let img: String = manipulation::convert("pikachu_p.jpg");
        // manipulation::convert("test.png");
        match manipulation::save_redis(img) {
            Ok(_) => println!("Sucesso no redis"),
            Err(_) => println!("Sem sucesso no redis"),
        };
    }

}
