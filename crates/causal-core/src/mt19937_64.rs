//! Shared MT19937-64 engine extracted unchanged from the ranger port.
//! Distribution and shuffle algorithms remain specific to each oracle.
#[derive(Clone)]
pub(crate) struct Mt19937_64 {
    state: [u64; 312],
    index: usize,
}
impl Mt19937_64 {
    pub(crate) fn new(seed: u64) -> Self {
        let mut state = [0; 312];
        state[0] = seed;
        for i in 1..312 {
            state[i] = 6364136223846793005_u64
                .wrapping_mul(state[i - 1] ^ (state[i - 1] >> 62))
                .wrapping_add(i as u64);
        }
        Self { state, index: 312 }
    }
    pub(crate) fn next(&mut self) -> u64 {
        if self.index == 312 {
            for i in 0..312 {
                let joined =
                    (self.state[i] & 0xffffffff80000000) | (self.state[(i + 1) % 312] & 0x7fffffff);
                self.state[i] = self.state[(i + 156) % 312]
                    ^ (joined >> 1)
                    ^ if joined & 1 != 0 {
                        0xb5026f5aa96619e9
                    } else {
                        0
                    };
            }
            self.index = 0;
        }
        let mut value = self.state[self.index];
        self.index += 1;
        value ^= (value >> 29) & 0x5555555555555555;
        value ^= (value << 17) & 0x71d67fffeda60000;
        value ^= (value << 37) & 0xfff7eee000000000;
        value ^ (value >> 43)
    }
}
