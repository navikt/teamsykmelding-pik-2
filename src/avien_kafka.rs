use rdkafka::ClientConfig;
use rdkafka::consumer::{BaseConsumer, Consumer};
use crate::environment_variables::EnvironmentVariables;

pub fn avien_kafka(environment_variables: EnvironmentVariables) {
    println!("Staring to setup kafka config");


    // kafka config
    let intern_pik_topic: [&str; 1] = [environment_variables.intern_pik_topic];
    let kafka_client_id = environment_variables.hostname + "-paragraf-i-kode";
    let application_name = environment_variables.application_name;

    let kafka_brokers: String = environment_variables.kafka_brokers;
    let kafka_certificate_path: String = environment_variables.kafka_certificate_path;
    let kafka_private_key_path: String = environment_variables.kafka_private_key_path;
    let kafka_ca_path: String = environment_variables.kafka_ca_path;


    let kafka_consumer: BaseConsumer = ClientConfig::new()
        .set("bootstrap.servers", kafka_brokers)
        .set("group.id", application_name)
        .set("client.id", kafka_client_id)
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        .set("security.protocol", "SSL")
        .set("auto.offset.reset", "earliest")
        .set("ssl.key.location", kafka_private_key_path)
        .set("ssl.certificate.location", kafka_certificate_path)
        .set("ssl.ca.location", kafka_ca_path)
        .create()
        .expect("Consumer creation error");

    kafka_consumer.subscribe(intern_pik_topic).expect("TODO: panic message");

    println!("made it passed kafka config");

    loop {
        let message = kafka_consumer.poll(None);
        println!("{:?}", message);
    }

}
