use shaku::Interface;

use crate::error::AppResult;

pub trait EntropyService: Interface {
    fn next_bool(&mut self) -> AppResult<bool>;
    fn next_u8(&mut self) -> AppResult<u8>;
    fn next_u16(&mut self) -> AppResult<u16>;
    fn next_u32(&mut self) -> AppResult<u32>;
    fn next_u64(&mut self) -> AppResult<u64>;
    fn next_u128(&mut self) -> AppResult<u128>;

    fn next_u8_ranged(&mut self, min: u8, max: u8) -> AppResult<u8>;
    fn next_u16_ranged(&mut self, min: u16, max: u16) -> AppResult<u16>;
    fn next_u32_ranged(&mut self, min: u32, max: u32) -> AppResult<u32>;
    fn next_u64_ranged(&mut self, min: u64, max: u64) -> AppResult<u64>;
    fn next_u128_ranged(&mut self, min: u128, max: u128) -> AppResult<u128>;

    fn next_bytes_inplace(&mut self, buf: &mut [u8]) -> AppResult<()>;
    fn next_bytes(&mut self, len: usize) -> AppResult<Vec<u8>>;

    fn next_string(&mut self, len: usize) -> AppResult<String>;
    fn next_string_with(
        &mut self,
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
