use rand::{Rng, SeedableRng};

pub struct Random {
    random: rand::rngs::StdRng,
}

impl Random {
    pub fn gen_u32(&mut self) -> u32 {
        self.random.gen()
    }

    pub fn gen_u64(&mut self) -> u64 {
        self.random.gen()
    }

    pub fn gen_u8(&mut self) -> u8 {
        self.random.gen()
    }

    /// # Panic
    /// If min is bigger than max, will panic.
    pub fn gen_u32_in_range(&mut self, min: u32, max: u32) -> u32 {
        if min > max {
            panic!("Minimum is bigger than maximum.");
        }
        min + self.gen_u32() % (max - min + 1)
    }

    /// # Panic
    /// If min is bigger than max, will panic.
    pub fn gen_u8_in_range(&mut self, min: u8, max: u8) -> u8 {
        if min > max {
            panic!("Minimum is bigger than maximum.");
        }
        min + self.gen_u8() % (max - min + 1)
    }

    pub fn new() -> Self {
        Self {
            random: rand::rngs::StdRng::from_entropy(),
        }
    }
}
