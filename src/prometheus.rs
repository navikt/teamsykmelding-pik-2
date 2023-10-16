use prometheus::{self};

use prometheus::{register_int_counter, IntCounter};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref PRODUCED_MGS: IntCounter = register_int_counter!(
        "teamsykmelding_pik_produced_msg_counter",
        "Number of messages produced"
    )
    .unwrap();
}