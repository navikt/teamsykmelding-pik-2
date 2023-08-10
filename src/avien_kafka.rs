use std::collections::HashMap;
use rdkafka::{ClientConfig, Message};
use rdkafka::consumer::{BaseConsumer, Consumer};
use serde_derive::{Deserialize, Serialize};
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
        .set("group.id", application_name + "-consumer")
        .set("client.id", kafka_client_id)
        .set("session.timeout.ms", "6000")
        .set("auto.offset.reset", "earliest")
        .set("enable.auto.commit", "false")
        .set("security.protocol", "ssl")
        .set("ssl.key.location", kafka_private_key_path)
        .set("ssl.certificate.location", kafka_certificate_path)
        .set("ssl.ca.location", kafka_ca_path)
        .create()
        .expect("Failed to create Kafka consumer");

    kafka_consumer.subscribe(intern_pik_topic.as_ref()).expect("We could not subscribe to the defined topic.");

    println!("made it passed kafka config");

    loop {
        let msg_result = kafka_consumer.poll(None).unwrap();

        let msg = msg_result.unwrap();
        let payload = msg.payload().unwrap();

        let payload_json_string = std::str::from_utf8(payload).unwrap();

        let juridisk_vurdering_result: JuridiskVurderingResult = serde_json::from_str(payload_json_string).unwrap();

        println!("juridisk_vurdering_result is: {:?}", juridisk_vurdering_result);

        kafka_consumer.commit_message(&msg, rdkafka::consumer::CommitMode::Sync).unwrap();
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct JuridiskVurderingResult {
    pub(crate) juridiskeVurderinger: Vec<JuridiskVurdering>,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct JuridiskVurdering {
    pub(crate) id: String,
    pub(crate) eventName: String,
    pub(crate) version: String,
    pub(crate) kilde: String,
    pub(crate) versjonAvKode: String,
    pub(crate) fodselsnummer: String,
    pub(crate) juridiskHenvisning: JuridiskHenvisning,
    pub(crate) sporing: HashMap<String, Option<String>>,
    pub(crate) input: HashMap<String, String>,
    pub(crate) tidsstempel: Option<String>,
    pub(crate) utfall: JuridiskUtfall,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct JuridiskHenvisning {
    pub(crate) lovverk: Lovverk,
    pub(crate) paragraf: String,
    pub(crate) ledd: Option<u32>,
    pub(crate) punktum: Option<u32>,
    pub(crate) bokstav: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_camel_case_types)]
pub enum JuridiskUtfall {
    VILKAR_OPPFYLT,
    VILKAR_IKKE_OPPFYLT,
    VILKAR_UAVKLART,
    VILKAR_BEREGNET,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_camel_case_types)]
pub enum Lovverk {
    FOLKETRYGDLOVEN,
    FORVALTNINGSLOVEN,
}
