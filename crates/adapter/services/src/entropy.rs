use std::sync::{Arc, Mutex, MutexGuard};

use kernel_services::{
    entropy::{CharacterCase, EntropyService, RandomStringOptions},
    error::{AppError, AppResult},
    Service,
};
use rand::{Rng, RngCore};

const UPPER_ALPHA: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const LOWER_ALPHA: &str = "abcdefghijklmnopqrstuvwxyz";
const MIXED_ALPHA: &str =
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
const NUMERAL: &str = "0123456789";
const SPECIAL: &str = "`~!@#$%^&*()-_=+[]{};:'\"|\\,.<>/?";

pub type BasicEntropyService = EntropyServiceImpl<rand::rngs::SmallRng>;
pub type SecureEntropyService = EntropyServiceImpl<rand::rngs::OsRng>;

#[derive(Default)]
pub struct EntropyServiceImpl<R>(Arc<Mutex<R>>);

impl<R: RngCore + Send + Sync> EntropyService for EntropyServiceImpl<R> {
    fn next_bool(&self) -> AppResult<bool> {
        Ok(self.inner()?.gen())
    }

    fn next_u8(&self) -> AppResult<u8> {
        Ok(self.inner()?.gen())
    }

    fn next_u16(&self) -> AppResult<u16> {
        Ok(self.inner()?.gen())
    }

    fn next_u32(&self) -> AppResult<u32> {
        Ok(self.inner()?.gen())
    }

    fn next_u64(&self) -> AppResult<u64> {
        Ok(self.inner()?.gen())
    }

    fn next_u128(&self) -> AppResult<u128> {
        Ok(self.inner()?.gen())
    }

    fn next_usize(&self) -> AppResult<usize> {
        Ok(self.inner()?.gen())
    }

    fn next_u8_ranged(&self, min: u8, max: u8) -> AppResult<u8> {
        Ok(self.inner()?.gen_range(min..max))
    }

    fn next_u16_ranged(&self, min: u16, max: u16) -> AppResult<u16> {
        Ok(self.inner()?.gen_range(min..max))
    }

    fn next_u32_ranged(&self, min: u32, max: u32) -> AppResult<u32> {
        Ok(self.inner()?.gen_range(min..max))
    }

    fn next_u64_ranged(&self, min: u64, max: u64) -> AppResult<u64> {
        Ok(self.inner()?.gen_range(min..max))
    }

    fn next_u128_ranged(&self, min: u128, max: u128) -> AppResult<u128> {
        Ok(self.inner()?.gen_range(min..max))
    }

    fn next_usize_ranged(&self, min: usize, max: usize) -> AppResult<usize> {
        Ok(self.inner()?.gen_range(min..max))
    }

    fn next_bytes_inplace(&self, buf: &mut [u8]) -> AppResult<()> {
        self.inner()?
            .try_fill_bytes(buf)
            .map_err(|err| AppError::Unknown(err.into()))?;

        Ok(())
    }

    fn next_bytes(&self, len: usize) -> AppResult<Vec<u8>> {
        let mut buf: Vec<u8> = Vec::with_capacity(len);

        self.next_bytes_inplace(&mut buf)?;

        Ok(buf)
    }

    fn next_string(&self, len: usize) -> AppResult<String> {
        self.next_string_with(len, Default::default())
    }

    fn next_string_with(
        &self,
        len: usize,
        opts: RandomStringOptions,
    ) -> AppResult<String> {
        let mut pool = String::new();

        if let Some(ctype) = opts.alpha {
            pool.push_str(match ctype {
                | CharacterCase::Lower => LOWER_ALPHA,
                | CharacterCase::Upper => UPPER_ALPHA,
                | CharacterCase::Mixed => MIXED_ALPHA,
            });
        }

        if opts.numeric {
            pool.push_str(NUMERAL);
        }

        if opts.special {
            pool.push_str(SPECIAL);
        }

        let mut out = String::with_capacity(len);

        for _ in 0..len {
            let rnd_chr = pool
                .chars()
                .nth(self.next_usize_ranged(0, pool.len() - 1)?)
                .unwrap();
            out.push(rnd_chr);
        }

        Ok(out)
    }
}

impl<R> EntropyServiceImpl<R> {
    fn inner(&self) -> AppResult<MutexGuard<'_, R>> {
        let lck = self
            .0
            .lock()
            .map_err(|err| anyhow::anyhow!(err.to_string()))?;

        Ok(lck)
    }
}

#[async_trait::async_trait]
impl<R: Send + Sync> Service for EntropyServiceImpl<R> {
    async fn initialize(self: Arc<Self>) -> AppResult<()> {
        Ok(())
    }
}
