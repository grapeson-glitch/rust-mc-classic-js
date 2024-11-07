pub struct Random {
    pub rand: i32
}

impl Random {
    pub fn new (mut seed: i32) -> Self {
        seed %= 2147483647;
        if seed <= 0 {seed += 2147483646;}
        Random {
            rand: seed
        }
    }

    /**
    * Returns a pseudo-random value between 1 and 2^32 - 2.
    */
    pub fn next (&mut self) -> i32 {
        self.rand = self.rand * 16807 % 2147483647;
        return self.rand;
    }

    pub fn nextInt (&mut self, max: i32) -> i32 {
        let f0: f64 = self.nextFloat() * max as f64;
        return f0.floor() as i32;
    }

    /**
    * Returns a pseudo-random floating point number in range [0, 1).
    */
    pub fn nextFloat (&mut self) -> f64 {
        // We know that result of next() will be 1 to 2147483646 (inclusive).
        return (self.next()-1) as f64 / 2147483646.0;
    }


}
