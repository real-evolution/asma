use validator::ValidationError;
use validators::traits::ValidateString;

use crate::parse::*;

fn validate<V: ValidateString>(
    code: &'static str,
    value: &str,
) -> Result<(), ValidationError> {
    if V::parse_str(value).is_ok() {
        return Ok(());
    }

    Err(ValidationError::new(code))
}

pub fn host(value: &str) -> Result<(), ValidationError> {
    validate::<Host>("host", value)
}

pub fn endpoint(value: &str) -> Result<(), ValidationError> {
    validate::<Endpoint>("endpoint", value)
}

pub fn ip_endpoint(value: &str) -> Result<(), ValidationError> {
    validate::<IpEndpoint>("ip_endpoint", value)
}

pub fn supported_driver(value: &str) -> Result<(), ValidationError> {
    validate::<SupportedDriver>("supported_driver", value)
}

pub fn identifier(value: &str) -> Result<(), ValidationError> {
    validate::<Identifier>("identifier", value)
}

pub fn username(value: &str) -> Result<(), ValidationError> {
    validate::<Username>("username", value)
}
