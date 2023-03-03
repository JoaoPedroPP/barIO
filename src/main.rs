use std::{env, time::Duration};
use rdkafka::{
    consumer::{BaseConsumer, Consumer},
    ClientConfig, Message
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
        .set("sasl.username", kafka_username)
        .set("sasl.password", kafka_password)
        .set("security.protocol", "SASL_SSL")
        .set("sasl.mechanisms", "PLAIN")
        .set("group.id", kafka_group)
        .create()
        .expect("Invalid Kafka Consumer Config");

    consumer
        .subscribe(&[&kafka_consumer_topic])
        .expect("Consumer topic failed");

    loop {
        let pool = consumer.poll(Duration::from_secs(1));
        // println!("Pooling");
        for msg in pool {
            let raw = msg.unwrap();
            let bytes = raw.payload().unwrap();
            let payload: manipulation::KafkaConsumerPayload = bincode::deserialize(bytes).unwrap();
            println!("Payload: {:?}", payload);
            let img: String = manipulation::convert(&payload.img);
            match manipulation::save_redis(payload.key, img) {
                Ok(_) => println!("Sucesso no redis"),
                Err(_) => println!("Sem sucesso no redis"),
            };
        }
    }
}
