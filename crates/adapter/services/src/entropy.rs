use kernel_services::entropy::{
    CharacterCase, EntropyService, RandomStringOptions,
};
use kernel_services::error::{AppError, AppResult};
use rand::{distributions::Uniform, Rng, RngCore};
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
    rng: R,
}

impl<R: RngCore + Send + Sync> EntropyService for EntropyServiceImpl<R> {
    fn next_bool(&mut self) -> AppResult<bool> {
        Ok(self.rng.gen())
    }

    fn next_u8(&mut self) -> AppResult<u8> {
        Ok(self.rng.gen())
    }

    fn next_u16(&mut self) -> AppResult<u16> {
        Ok(self.rng.gen())
    }

    fn next_u32(&mut self) -> AppResult<u32> {
        Ok(self.rng.gen())
    }

    fn next_u64(&mut self) -> AppResult<u64> {
        Ok(self.rng.gen())
    }

    fn next_u128(&mut self) -> AppResult<u128> {
        Ok(self.rng.gen())
    }

    fn next_u8_ranged(&mut self, min: u8, max: u8) -> AppResult<u8> {
        Ok(self.rng.gen_range(min..max))
    }

    fn next_u16_ranged(&mut self, min: u16, max: u16) -> AppResult<u16> {
        Ok(self.rng.gen_range(min..max))
    }

    fn next_u32_ranged(&mut self, min: u32, max: u32) -> AppResult<u32> {
        Ok(self.rng.gen_range(min..max))
    }

    fn next_u64_ranged(&mut self, min: u64, max: u64) -> AppResult<u64> {
        Ok(self.rng.gen_range(min..max))
    }

    fn next_u128_ranged(&mut self, min: u128, max: u128) -> AppResult<u128> {
        Ok(self.rng.gen_range(min..max))
    }

    fn next_bytes_inplace(&mut self, buf: &mut [u8]) -> AppResult<()> {
        self.rng
            .try_fill_bytes(buf)
            .map_err(|err| AppError::Unknown(err.into()))?;

        Ok(())
    }

    fn next_bytes(&mut self, len: usize) -> AppResult<Vec<u8>> {
        let mut buf: Vec<u8> = Vec::with_capacity(len);

        self.next_bytes_inplace(&mut buf)?;

        Ok(buf)
    }

    fn next_string(&mut self, len: usize) -> AppResult<String> {
        Ok(self.next_string_with(len, Default::default())?)
    }

    fn next_string_with(
        &mut self,
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

        let rng = &mut self.rng;

        Ok(rng
            .sample_iter(Uniform::new(0, pool.len()))
            .map(|i| pool.chars().nth(i).unwrap())
            .take(len)
            .collect())
    }
}
