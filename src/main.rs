mod handle_client;
mod environment_variables;

use std::fs::File;
use std::net::{TcpListener};
use std::string::ToString;
use kafka::client::{KafkaClient, SecurityConfig};
use serde_derive::{Deserialize, Serialize};
use crate::environment_variables::get_environment_variables;
use crate::handle_client::handle_client;
use kafka::consumer::{Consumer, FetchOffset};
use openssl::ssl::{SslConnector, SslFiletype, SslMethod, SslVerifyMode};


fn main() {
    //start server and print port
    let listener = TcpListener::bind(format!("0.0.0.0:8080")).unwrap();
    println!("Server listening on port 8080");

    let environment_variables = get_environment_variables();

    println!("cluster_name is : {}", environment_variables.cluster_name.to_string());


    let application_state = ApplicationState {
        alive: true,
        ready: true,
    };

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream, application_state);
            }
            Err(e) => {
                println!("Unable to connect: {}", e);
            }
        }
    }

    // setup ssl config for kafka aiven
    let kafka_brokers = environment_variables.kafka_brokers.to_string();
    let kafka_certificate_path = environment_variables.kafka_certificate_path.to_string();
    let kafka_private_key_path = environment_variables.kafka_private_key_path.to_string();
    let kafka_ca_path = environment_variables.kafka_ca_path.to_string();

    let certificate_file = File::open(kafka_certificate_path);
    let private_key_file = File::open(kafka_private_key_path);
    let ca_file = File::open(kafka_ca_path);

    let mut ssl_connector_builder = SslConnector::builder(SslMethod::tls()).unwrap();
    ssl_connector_builder.set_cipher_list("DEFAULT").unwrap();
    ssl_connector_builder.set_verify(SslVerifyMode::PEER).unwrap();
    ssl_connector_builder
        .set_certificate_file(certificate_file, SslFiletype::PEM)
        .unwrap();
    ssl_connector_builder
        .set_private_key_file(private_key_file, SslFiletype::PEM)
        .unwrap();
    ssl_connector_builder.set_ca_file(ca_file).unwrap();


    let ssl_connector = ssl_connector_builder.build();

    let kafka_client: KafkaClient = KafkaClient::new_secure(
        vec!(kafka_brokers.to_owned(), &[0, 1], ),
        (SecurityConfig::new(ssl_connector).with_hostname_verification(true)));

    // kafka config
    let intern_pik_topic = environment_variables.intern_pik_topic.to_string();
    let kafka_hostname = environment_variables.kafka_hostname.to_string() + "-paragraf-i-kode";
    let application_name = environment_variables.application_name.to_string();

    // start to consume kafka messeges
    let mut kafka_consumer =
        Consumer::from_client(kafka_client)
            .with_fallback_offset(FetchOffset::Latest)
            .with_topic_partitions(intern_pik_topic.to_owned(), &[0, 1])
            .with_group(application_name.to_owned())
            .with_client_id(kafka_hostname)
            .create()
            .unwrap();

    loop {
        for messageSet in kafka_consumer.poll().unwrap().iter() {
            for message in messageSet.messages() {
                println!("{:?}", message);
            }
            kafka_consumer.consume_messageset(messageSet).expect("panic message");
        }
        kafka_consumer.commit_consumed().unwrap();
    }
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct ApplicationState {
    alive: bool,
    ready: bool,
}
