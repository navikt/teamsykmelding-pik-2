use std::collections::HashMap;
use rdkafka::{ClientConfig, Message};
use rdkafka::consumer::{BaseConsumer, Consumer};
use rdkafka::producer::{BaseProducer, BaseRecord};
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use uuid::{Uuid};
use crate::environment_variables::EnvironmentVariables;

pub fn avien_kafka(environment_variables: EnvironmentVariables) {
    let intern_pik_topic: [&str; 1] = [environment_variables.intern_pik_topic];
    let etterlevelse_topic: &str = environment_variables.etterlevelse_topic;

    let kafka_client_id = environment_variables.hostname + "-paragraf-i-kode";
    let application_name = environment_variables.application_name;

    let kafka_brokers: String = environment_variables.kafka_brokers;
    let kafka_certificate_path: String = environment_variables.kafka_certificate_path;
    let kafka_private_key_path: String = environment_variables.kafka_private_key_path;
    let kafka_ca_path: String = environment_variables.kafka_ca_path;

    let kafka_consumer: BaseConsumer = ClientConfig::new()
        .set("bootstrap.servers", kafka_brokers.clone())
        .set("group.id", application_name.clone() + "-consumer")
        .set("client.id", kafka_client_id.clone())
        .set("session.timeout.ms", "6000")
        .set("auto.offset.reset", "earliest")
        .set("enable.auto.commit", "false")
        .set("security.protocol", "ssl")
        .set("ssl.key.location", kafka_private_key_path.clone())
        .set("ssl.certificate.location", kafka_certificate_path.clone())
        .set("ssl.ca.location", kafka_ca_path.clone())
        .create()
        .expect("Failed to create Kafka consumer");

    kafka_consumer.subscribe(intern_pik_topic.as_ref()).expect("We could not subscribe to the defined topic.");

    let kafka_producer: BaseProducer = ClientConfig::new()
        .set("bootstrap.servers", kafka_brokers)
        .set("group.id", application_name + "-producer")
        .set("client.id", kafka_client_id)
        .set("session.timeout.ms", "6000")
        .set("auto.offset.reset", "earliest")
        .set("enable.auto.commit", "false")
        .set("security.protocol", "ssl")
        .set("ssl.key.location", kafka_private_key_path)
        .set("ssl.certificate.location", kafka_certificate_path)
        .set("ssl.ca.location", kafka_ca_path)
        .create()
        .expect("Failed to create Kafka producer");

    loop {
        let msg_result = kafka_consumer.poll(None).unwrap();
        let msg = msg_result.unwrap();
        let payload = msg.payload().unwrap();
        let payload_as_json_string = std::str::from_utf8(payload).unwrap();
        let juridisk_vurdering_result: JuridiskVurderingResult = serde_json::from_str(payload_as_json_string).unwrap();
        kafka_consumer.commit_message(&msg, rdkafka::consumer::CommitMode::Sync).unwrap();

        for juridiske_vurderinger in juridisk_vurdering_result.juridiskeVurderinger {
            let juridisk_vurdering_kafka_message = JuridiskVurderingKafkaMessage {
                id: Uuid::new_v4(),
                tidsstempel: juridiske_vurderinger.tidsstempel.unwrap(),
                eventName: juridiske_vurderinger.eventName.unwrap(),
                versjon: juridiske_vurderinger.versjon.unwrap(),
                kilde: juridiske_vurderinger.kilde.unwrap(),
                versjonAvKode: juridiske_vurderinger.versjonAvKode.unwrap(),
                fodselsnummer: juridiske_vurderinger.fodselsnummer.unwrap(),
                sporing: Default::default(),
                lovverk: juridiske_vurderinger.fodselsnummer.unwrap(),
                lovverksversjon: juridiske_vurderinger.lovverk.lovverksversjon.unwrap(),
                paragraf: juridiske_vurderinger.paragraf.unwrap(),
                ledd: juridiske_vurderinger.ledd.unwrap(),
                punktum: juridiske_vurderinger.punktum.unwrap(),
                bokstav: juridiske_vurderinger.bokstav.unwrap(),
                input: juridiske_vurderinger.input,
                output: None,
                utfall: Utfall::VILKAR_OPPFYLT,
            };

            let juridisk_vurdering_kafka_message_json =
                serde_json::to_string_pretty(&juridisk_vurdering_kafka_message)
                    .expect("json serialization faileid");

            kafka_producer.send(
                BaseRecord::to(etterlevelse_topic)
                    .key(&[1, 2, 3, 4])
                    .payload(&juridisk_vurdering_kafka_message_json),
            ).expect("Failed to send message");
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct JuridiskVurderingKafkaMessage {
    pub(crate) id: Uuid,
    pub(crate) tidsstempel: String,
    pub(crate) eventName: String,
    pub(crate) versjon: String,
    pub(crate) kilde: String,
    pub(crate) versjonAvKode: String,
    pub(crate) fodselsnummer: String,
    pub(crate) sporing: HashMap<String, Vec<String>>,
    pub(crate) lovverk: String,
    pub(crate) lovverksversjon: String,
    pub(crate) paragraf: String,
    pub(crate) ledd: Option<u32>,
    pub(crate) punktum: Option<u32>,
    pub(crate) bokstav: Option<String>,
    pub(crate) input: HashMap<String, Value>,
    pub(crate) output: Option<HashMap<String, Value>>,
    pub(crate) utfall: Utfall,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_camel_case_types)]
pub enum Utfall {
    VILKAR_OPPFYLT,
    VILKAR_IKKE_OPPFYLT,
    VILKAR_UAVKLART,
    VILKAR_BEREGNET,
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
    pub(crate) sporing: HashMap<String, String>,
    pub(crate) input: HashMap<String, Value>,
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
