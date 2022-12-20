use argon2::{
    password_hash::{Error, SaltString},
    PasswordHash,
    PasswordHasher,
    PasswordVerifier,
};
use kernel_services::{
    crypto::hash::CryptoHashService,
    error::{AppResult, CryptoError},
    Service,
};
use rand::rngs::OsRng;

pub type Argon2CryptoHashService<'a> =
    CryptoHashServiceImpl<argon2::Argon2<'a>>;

impl<'a> Argon2CryptoHashService<'a> {
    pub fn new() -> Self {
        Self(argon2::Argon2::default())
    }
}

pub struct CryptoHashServiceImpl<H>(H);

impl<H> CryptoHashService for CryptoHashServiceImpl<H>
where
    H: PasswordHasher + PasswordVerifier + Sync + Send,
{
    fn hash(&self, plain: &str) -> kernel_services::error::AppResult<String> {
        let salt = SaltString::generate(&mut OsRng);

        Ok(self
            .0
            .hash_password(plain.as_bytes(), &salt)
            .map_err(map_hash_error)?
            .to_string())
    }

    fn verify(&self, plain: &str, hash: &str) -> AppResult<()> {
        let hash = PasswordHash::new(hash).map_err(map_hash_error)?;

        Ok(self
            .0
            .verify_password(plain.as_bytes(), &hash)
            .map_err(map_hash_error)?)
    }
}

impl<H> Service for CryptoHashServiceImpl<H> {}

fn map_hash_error(err: Error) -> CryptoError {
    match err {
        | Error::Algorithm | Error::Version => CryptoError::Unsupported,
        | Error::B64Encoding(err) => CryptoError::Encoding(err.to_string()),
        | Error::Crypto => CryptoError::Format("crypto error".into()),
        | Error::OutputTooShort => CryptoError::InputTooShort,
        | Error::OutputTooLong => CryptoError::InputTooLong,
        | Error::ParamNameDuplicated
        | Error::ParamNameInvalid
        | Error::ParamValueInvalid(_)
        | Error::PhcStringInvalid
        | Error::PhcStringTooShort
        | Error::PhcStringTooLong
        | Error::ParamsMaxExceeded => CryptoError::InvalidInput,
        | Error::Password => CryptoError::Verification(
            "hash does not corrospond to the input".into(),
        ),
        | Error::SaltInvalid(err) => CryptoError::Salt(err.to_string()),

        | _ => CryptoError::Hash("unknown error".into()),
    }
}
