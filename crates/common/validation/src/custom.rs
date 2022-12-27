use chrono::{DateTime, Utc};
use validator::ValidationError;

use crate::{
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

pub fn in_future(value: &DateTime<Utc>) -> Result<(), ValidationError> {
    if value < &Utc::now() {
        return Err(ValidationError::new("value cannot be in the past"));
    }
pub fn supported_data_driver(value: &str) -> Result<(), ValidationError> {
    validate_with("supported_data_driver", value, |v| {
        SUPPORTED_DATA_DRIVERS.contains(&v)
    })
}

    Ok(())
}
