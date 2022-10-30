use crate::error::AppResult;

pub trait EntropyService {
    fn next_u8(&self) -> AppResult<u8>;
    fn next_u16(&self) -> AppResult<u16>;
    fn next_u32(&self) -> AppResult<u32>;
    fn next_u64(&self) -> AppResult<u64>;
    fn next_u128(&self) -> AppResult<u128>;
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
