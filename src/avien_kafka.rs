use std::path::Path;
use kafka::client::{FetchOffset, GroupOffsetStorage, KafkaClient, SecurityConfig};
use kafka::consumer::Consumer;
use openssl::ssl::{SslConnector, SslFiletype, SslMethod, SslVerifyMode};
use crate::environment_variables::EnvironmentVariables;

pub fn avien_kafka(environment_variables : EnvironmentVariables) {
    println!("Staring to setup kafka config");

    // setup ssl config for kafka aiven
    let kafka_brokers: String = environment_variables.kafka_brokers;
    let kafka_certificate_path: String = environment_variables.kafka_certificate_path;
    let kafka_private_key_path: String = environment_variables.kafka_private_key_path;
    let kafka_ca_path: String = environment_variables.kafka_ca_path;


    let mut ssl_connector_builder = SslConnector::builder(SslMethod::tls()).unwrap();
    ssl_connector_builder.set_cipher_list("DEFAULT").unwrap();
    ssl_connector_builder.set_verify(SslVerifyMode::PEER);
    ssl_connector_builder
        .set_certificate_file(Path::new(kafka_certificate_path.as_str()) , SslFiletype::PEM)
        .unwrap();
    ssl_connector_builder
        .set_private_key_file(Path::new(kafka_private_key_path.as_str()), SslFiletype::PEM)
        .unwrap();
    ssl_connector_builder.set_ca_file(Path::new(kafka_ca_path.as_str())).unwrap();


    let ssl_connector = ssl_connector_builder.build();

    let kafka_client: KafkaClient = KafkaClient::new_secure(
        vec!(kafka_brokers),
        SecurityConfig::new(ssl_connector).with_hostname_verification(true));

    // kafka config
    let intern_pik_topic = environment_variables.intern_pik_topic;
    let kafka_client_id = environment_variables.hostname + "-paragraf-i-kode";
    let application_name = environment_variables.application_name;

    println!("made it passed kafka config");

    // start to consume kafka messeges
    let mut kafka_consumer =
        Consumer::from_client(kafka_client)
            .with_fallback_offset(FetchOffset::Latest)
            .with_topic(intern_pik_topic.to_owned())
            .with_group(application_name.to_owned())
            .with_client_id(kafka_client_id)
            .with_offset_storage(GroupOffsetStorage::Kafka)
            .create()
            .unwrap();

    println!("made it to kafka consumer loop");

    loop {
        for message_set in kafka_consumer.poll().unwrap().iter() {
            for message in message_set.messages() {
                println!("{:?}", message);
            }
            kafka_consumer.consume_messageset(message_set).unwrap();
            println!("message is consumed");
        }
        kafka_consumer.commit_consumed().unwrap();
    }
}