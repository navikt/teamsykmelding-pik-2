use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use log::{info, warn};
use rdkafka::{ClientConfig, Message};
use rdkafka::consumer::{BaseConsumer, Consumer};
use rdkafka::producer::{BaseProducer, BaseRecord};

use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use uuid::{Uuid};
use crate::avien_kafka::Lovverk::{FOLKETRYGDLOVEN, FORVALTNINGSLOVEN, HELSEPERSONELLOVEN};
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

    kafka_consumer.subscribe(intern_pik_topic.as_ref()).expect("We could not subscribe to the topic");

    let kafka_producer: BaseProducer = ClientConfig::new()
        .set("bootstrap.servers", kafka_brokers)
        .set("client.id", kafka_client_id)
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

        let juridiske_vurderinger_first = juridisk_vurdering_result.juridiskeVurderinger.first().clone();

        match juridiske_vurderinger_first {
            None => {
                warn!("Failed to get juridiske_vurderinger_first variabel")
            }
            _ => info!("Consumed message from kafka topic sporingsinfo: {:?}", juridiske_vurderinger_first.unwrap().sporing)
        }

        for juridiske_vurderinger in juridisk_vurdering_result.juridiskeVurderinger {
            let juridisk_vurdering_kafka_message = JuridiskVurderingKafkaMessage {
                id: Uuid::new_v4(),
                tidsstempel: juridiske_vurderinger.tidsstempel,
                eventName: juridiske_vurderinger.eventName,
                versjon: juridiske_vurderinger.version,
                kilde: juridiske_vurderinger.kilde,
                versjonAvKode: juridiske_vurderinger.versjonAvKode,
                fodselsnummer: juridiske_vurderinger.fodselsnummer,
                sporing: map_to_sporing(juridiske_vurderinger.sporing),
                lovverk: juridiske_vurderinger.juridiskHenvisning.lovverk.get_lovverk_kortnavn_kafka_message().unwrap().to_lowercase(),
                lovverksversjon: juridiske_vurderinger.juridiskHenvisning.lovverk.get_lovverk_lovverksversjon_kafka_message().unwrap(),
                paragraf: juridiske_vurderinger.juridiskHenvisning.paragraf,
                ledd: juridiske_vurderinger.juridiskHenvisning.ledd,
                punktum: juridiske_vurderinger.juridiskHenvisning.punktum,
                bokstav: juridiske_vurderinger.juridiskHenvisning.bokstav,
                input: juridiske_vurderinger.input,
                output: None,
                utfall: juridiske_vurderinger.utfall,
            };

            let juridisk_vurdering_kafka_message_json =
                serde_json::to_string_pretty(&juridisk_vurdering_kafka_message)
                    .expect("json serialization faileid");

            kafka_producer.send(
                BaseRecord::to(etterlevelse_topic)
                    .key(&juridisk_vurdering_kafka_message.clone().as_bytes())
                    .payload(&juridisk_vurdering_kafka_message_json),
            ).expect("Failed to send message");

            info!("Produced message to kafka topic sporingsinfo: {:?}", juridisk_vurdering_kafka_message.sporing.clone());

        }
    }
}

fn map_to_sporing(sporing: HashMap<String, String>) -> HashMap<String, Vec<String>> {
    let mut sporing_hash_map = HashMap::new();

    for key in sporing {
        let mut value: Vec<String> = Vec::new();
        value.push(key.1.clone());
        sporing_hash_map.insert(key.0.clone(), value);
    }

    return sporing_hash_map;

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
    pub(crate) utfall: JuridiskUtfall,
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
    pub(crate) tidsstempel: String,
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
    HELSEPERSONELLOVEN,
}


impl Lovverk {

    fn get_lovverk_kortnavn_kafka_message(&self) -> Result<String, Error> {
        match self {
            FOLKETRYGDLOVEN => Ok("Folketrygdloven".to_string()),
            FORVALTNINGSLOVEN => Ok("Forvaltningsloven".to_string()),
            HELSEPERSONELLOVEN => Ok("Helsepersonelloven".to_string()),
            _ => Err(Error::new(ErrorKind::InvalidData, "Unknow lovverk enum")),
        }
    }

    fn get_lovverk_lovverksversjon_kafka_message(&self) -> Result<String, Error> {
        match self {
            FOLKETRYGDLOVEN => Ok("2022-01-01".to_string()),
            FORVALTNINGSLOVEN => Ok("2022-01-01".to_string()),
            HELSEPERSONELLOVEN => Ok("2022-01-01".to_string()),
            _ => Err(Error::new(ErrorKind::InvalidData, "Unknow lovverk enum")),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::avien_kafka::map_to_sporing;

    #[test]
    fn test_map_to_sporing() {
        let mut sporing: HashMap<String, String> = HashMap::new();
        sporing.insert("sykmelding".to_string(), "12321-12313-123123".to_string());
        let sporing_kafka: HashMap<String, Vec<String>> = map_to_sporing(sporing);

        assert_eq!(sporing_kafka.values().len(), 1);
    }
}
