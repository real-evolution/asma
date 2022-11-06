use std::sync::{RwLock, RwLockWriteGuard};

use kernel_services::entropy::{
    CharacterCase, EntropyService, RandomStringOptions,
};
use kernel_services::error::{AppError, AppResult};
use rand::{Rng, RngCore};
use shaku::Component;

const UPPER_ALPHA: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const LOWER_ALPHA: &str = "abcdefghijklmnopqrstuvwxyz";
const MIXED_ALPHA: &str =
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
const NUMERAL: &str = "0123456789";
const SPECIAL: &str = "`~!@#$%^&*()-_=+[]{};:'\"|\\,.<>/?";

pub type BasicEntropyService = EntropyServiceImpl<rand::rngs::SmallRng>;
pub type SecureEntropyService = EntropyServiceImpl<rand::rngs::OsRng>;

#[derive(Component)]
#[shaku(interface = EntropyService)]
pub struct EntropyServiceImpl<R: RngCore + Send + Sync + 'static> {
    rng: WriteLock<R>,
}

impl<R: RngCore + Send + Sync> EntropyService for EntropyServiceImpl<R> {
    fn next_bool(&self) -> AppResult<bool> {
        Ok(self.rng.lock()?.gen())
    }

    fn next_u8(&self) -> AppResult<u8> {
        Ok(self.rng.lock()?.gen())
    }

    fn next_u16(&self) -> AppResult<u16> {
        Ok(self.rng.lock()?.gen())
    }

    fn next_u32(&self) -> AppResult<u32> {
        Ok(self.rng.lock()?.gen())
    }

    fn next_u64(&self) -> AppResult<u64> {
        Ok(self.rng.lock()?.gen())
    }

    fn next_u128(&self) -> AppResult<u128> {
        Ok(self.rng.lock()?.gen())
    }

    fn next_usize(&self) -> AppResult<usize> {
        Ok(self.rng.lock()?.gen())
    }

    fn next_u8_ranged(&self, min: u8, max: u8) -> AppResult<u8> {
        Ok(self.rng.lock()?.gen_range(min..max))
    }

    fn next_u16_ranged(&self, min: u16, max: u16) -> AppResult<u16> {
        Ok(self.rng.lock()?.gen_range(min..max))
    }

    fn next_u32_ranged(&self, min: u32, max: u32) -> AppResult<u32> {
        Ok(self.rng.lock()?.gen_range(min..max))
    }

    fn next_u64_ranged(&self, min: u64, max: u64) -> AppResult<u64> {
        Ok(self.rng.lock()?.gen_range(min..max))
    }

    fn next_u128_ranged(&self, min: u128, max: u128) -> AppResult<u128> {
        Ok(self.rng.lock()?.gen_range(min..max))
    }

    fn next_usize_ranged(&self, min: usize, max: usize) -> AppResult<usize> {
        Ok(self.rng.lock()?.gen_range(min..max))
    }

    fn next_bytes_inplace(&self, buf: &mut [u8]) -> AppResult<()> {
        self.rng
            .lock()?
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
        Ok(self.next_string_with(len, Default::default())?)
    }

    fn next_string_with(
        &self,
        len: usize,
        opts: RandomStringOptions,
    ) -> AppResult<String> {
        let mut pool = String::new();

        if let Some(ctype) = opts.alpha {
            pool.push_str(&match ctype {
                CharacterCase::Lower => LOWER_ALPHA,
                CharacterCase::Upper => UPPER_ALPHA,
                CharacterCase::Mixed => MIXED_ALPHA,
            });
        }

        if opts.numeric {
            pool.push_str(NUMERAL);
        }

        if opts.special {
            pool.push_str(SPECIAL);
        }

        let mut pool_chars = pool.chars();
        let mut out = String::with_capacity(len);

        for _ in 0..pool.len() {
            let rnd_chr = pool_chars.nth(self.next_usize()?).unwrap();
            out.push(rnd_chr);
        }

        Ok(out)
    }
}

pub struct WriteLock<T> {
    inner: RwLock<T>,
}

impl<T: Send + Sync> WriteLock<T> {
    fn lock(&self) -> AppResult<RwLockWriteGuard<T>> {
        match self.inner.try_write() {
            Ok(inner) => Ok(inner),
            Err(err) => {
                Err(AppError::Unknown(anyhow::anyhow!(err.to_string())))
            }
        }
    }
}

impl<T: Default> Default for WriteLock<T> {
    fn default() -> Self {
        Self { inner: Default::default() }
    }
}
