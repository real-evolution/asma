use validator::ValidationError;
use validators::traits::ValidateString;

pub(crate) fn validate<V: ValidateString>(
    code: &'static str,
    value: &str,
) -> Result<(), ValidationError> {
    if V::parse_str(value).is_ok() {
        return Ok(());
    }

    Err(ValidationError::new(code))
}

pub(crate) fn validate_with<T, F: Fn(T) -> bool>(
    code: &'static str,
    value: T,
    f: F,
) -> Result<(), ValidationError> {
    if f(value) {
        return Ok(());
    }

    Err(ValidationError::new(code))
}
