#[allow(unused)]
use crate::gates::*;
// SR Latch: Set-Reset latch with two inputs S and R.
#[derive(Clone, Copy)]
struct SRLatch {
    q: u8,
}

impl SRLatch {
    fn new() -> SRLatch {
        SRLatch { q: 0 }
    }

    fn update(&mut self, s: u8, r: u8) {
        match (s, r) {
            (1, 0) => self.q = 1, // Set
            (0, 1) => self.q = 0, // Reset
            (1, 1) => panic!("SR latch: Illegal state (both S and R are high)"),
            _ => {} // Maintain current state
        }
    }

    fn get_q(&self) -> u8 {
        self.q
    }
}

#[derive(Clone, Copy)]
pub struct DFlipFlop {
    q: u8,
    not_q: u8,
    sr_latch: SRLatch, // SR latch instance
}

impl DFlipFlop {
    pub fn new() -> DFlipFlop {
        DFlipFlop {
            q: 0,
            not_q: 1,
            sr_latch: SRLatch::new(),
        }
    }

    // Set the initial state of the D flip-flop
    pub fn set_initial_state(&mut self, initial_state: u8) {
        self.q = initial_state;
        self.not_q = not(initial_state); // Ensure `not` function also works with u8
    }

    // D Flip-Flop: captures the input `d` on the rising edge of the clock `clk`
    pub fn d_flipflop(&mut self, d: u8, clk: u8) {
        let s = and(d, clk); // Ensure `and` function works with u8
        let r = and(not(d), clk); // Ensure `not` function works with u8

        // Update the SR latch
        self.sr_latch.update(s, r);

        // Update the stored state using the SR latch's output
        self.q = self.sr_latch.get_q();
        self.not_q = not(self.q); // Ensure `not` function works with u8
    }

    // Returns the current value of Q
    pub fn output(&self) -> u8 {
        self.q
    }
}

#[derive(Clone, Copy)]
pub struct Bit {
    // 1 bit register
    dff: DFlipFlop,
}

impl Bit {
    pub fn new() -> Bit {
        Bit {
            dff: DFlipFlop::new(),
        }
    }

    // Updates the register based on the input, load, and clock
    pub fn bit(&mut self, input: u8, load: u8, clk: u8) {
        // Load determines if the input should be written
        let inp = mux(input, self.dff.output(), load); // Ensure `mux` works with u8
        self.dff.d_flipflop(inp, clk);
    }

    // Returns the current value of the register
    pub fn output(&self) -> u8 {
        self.dff.output()
    }

    // Read the individual bit of the register
    pub fn read(&self) -> u8 {
        self.dff.output()
    }

    // Write a new value to the individual bit of the register
    pub fn write(&mut self, value: u8) {
        self.dff.set_initial_state(value);
    }
}

pub struct Register {
    // 16 bit register
    bits: [Bit; 16],
}

impl Register {
    pub fn new() -> Register {
        Register {
            bits: [Bit::new(); 16],
        }
    }

    // Updates the register based on the input, load, and clock
    pub fn register(&mut self, input: [u8; 16], load: u8, clk: u8) {
        // Iterate over each bit of the input and update corresponding DFlipFlop
        for i in 0..16 {
            let inp = mux(input[i], self.bits[i].output(), load); // Ensure `mux` works with u8
            self.bits[i].bit(inp, load, clk);
        }
    }

    // Returns the current 16-bit value of the register
    pub fn output(&self) -> [u8; 16] {
        let mut res = [0u8; 16];
        for i in 0..16 {
            res[i] = self.bits[i].output();
        }
        res
    }

    // Read an individual bit of the register
    pub fn read(&self, index: usize) -> u8 {
        self.bits[index].read()
    }

    // Write a new value to an individual bit of the register
    pub fn write(&mut self, index: usize, value: u8) {
        self.bits[index].write(value);
    }
}
