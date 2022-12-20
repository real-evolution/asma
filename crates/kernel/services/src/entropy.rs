use crate::{error::AppResult, Service};

pub trait EntropyService: Service + Send + Sync {
    fn next_bool(&self) -> AppResult<bool>;
    fn next_u8(&self) -> AppResult<u8>;
    fn next_u16(&self) -> AppResult<u16>;
    fn next_u32(&self) -> AppResult<u32>;
    fn next_u64(&self) -> AppResult<u64>;
    fn next_u128(&self) -> AppResult<u128>;
    fn next_usize(&self) -> AppResult<usize>;

    fn next_u8_ranged(&self, min: u8, max: u8) -> AppResult<u8>;
    fn next_u16_ranged(&self, min: u16, max: u16) -> AppResult<u16>;
    fn next_u32_ranged(&self, min: u32, max: u32) -> AppResult<u32>;
    fn next_u64_ranged(&self, min: u64, max: u64) -> AppResult<u64>;
    fn next_u128_ranged(&self, min: u128, max: u128) -> AppResult<u128>;
    fn next_usize_ranged(&self, min: usize, max: usize) -> AppResult<usize>;

    fn next_bytes_inplace(&self, buf: &mut [u8]) -> AppResult<()>;
    fn next_bytes(&self, len: usize) -> AppResult<Vec<u8>>;

    fn next_string(&self, len: usize) -> AppResult<String>;
    fn next_string_with(
        &self,
        len: usize,
        opts: RandomStringOptions,
    ) -> AppResult<String>;
}

#[derive(Debug)]
pub struct RandomStringOptions {
    pub alpha: Option<CharacterCase>,
    pub numeric: bool,
    pub special: bool,
}

#[derive(Debug)]
pub enum CharacterCase {
    Lower,
    Upper,
    Mixed,
}

impl Default for RandomStringOptions {
    fn default() -> Self {
        Self {
            alpha: Some(CharacterCase::Mixed),
            numeric: true,
            special: false,
        }
    }
}
