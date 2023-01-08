use chrono::{DateTime, Utc};
use validator::ValidationError;

use crate::{
    constants::{
        SUPPORTED_DATA_DRIVERS, 
        SUPPORTED_MESSAGE_QUEUE_PROTOCOLS, SUPPORTED_DOC_STORE_DRIVERS,
    },
    helpers::{validate, validate_with},
    parse::*,
};

pub fn host(value: &str) -> Result<(), ValidationError> {
    validate::<Host>("host", value)
}

pub fn endpoint(value: &str) -> Result<(), ValidationError> {
    validate::<Endpoint>("endpoint", value)
}

pub fn ip_endpoint(value: &str) -> Result<(), ValidationError> {
    validate::<IpEndpoint>("ip_endpoint", value)
}
pub fn identifier(value: &str) -> Result<(), ValidationError> {
    validate::<Identifier>("identifier", value)
}

pub fn username(value: &str) -> Result<(), ValidationError> {
    validate::<Username>("username", value)
}

pub fn supported_data_driver(value: &str) -> Result<(), ValidationError> {
    validate_with("supported_data_driver", value, |v| {
        SUPPORTED_DATA_DRIVERS.contains(&v)
    })
}

pub fn supported_doc_store_driver(value: &str) -> Result<(), ValidationError> {
    validate_with("supported_doc_store_driver", value, |v| {
        SUPPORTED_DOC_STORE_DRIVERS.contains(&v)
    })
}

pub fn supported_message_queue_protocol(
    value: &str,
) -> Result<(), ValidationError> {
    validate_with("supported_message_queue_protocol", value, |v| {
        SUPPORTED_MESSAGE_QUEUE_PROTOCOLS.contains(&v)
    })
}

pub fn in_future(value: &DateTime<Utc>) -> Result<(), ValidationError> {
    validate_with("in_future", value, |ts| ts < &Utc::now())
}
