use std::env;
use rdkafka::{
    producer::{BaseProducer, BaseRecord},
    ClientConfig,
};

pub fn producer(data: &str) {
    let kafka_servers = env::var("KAFKA_SERVERS").expect("KAFKA_SERVERS not set");
    let kafka_username = env::var("KAFKA_USERNAME").expect("KAFKA_USERNAME not set");
    let kafka_password = env::var("KAFKA_PASSWORD").expect("KAFKA_PASSWORD not set");
    let kafka_producer_topic = env::var("KAFKA_PRODUCER_TOPIC").expect("KAFKA_PRODUCER_TOPIC not set");

    let producer: BaseProducer = ClientConfig::new()
        .set("bootstrap.servers", kafka_servers)
        .set("sasl.username", kafka_username)
        .set("sasl.password", kafka_password)
        .set("security.protocol", "SASL_SSL")
        .set("sasl.mechanisms", "PLAIN")
        .create()
        .expect("Invalid Kafka Producer Config");

    producer.send(
        BaseRecord::to(kafka_producer_topic)
            .key("msg")
            .payload(data)
            .expect("Msg could not be save")
    )
}